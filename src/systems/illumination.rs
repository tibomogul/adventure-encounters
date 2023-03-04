use crate::prelude::{*, map_builder::MapBuilder, distance::distance_between_points, tiles::Tile};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum IlluminationLevel {
    None,
    Dim,
    Normal
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct ProvidesIllumination {
    pub illuminated_tiles: HashSet<Point>,
    pub bright_interval: u16, // feet from origin
    pub shadowy_interval: u16, // feet, beyond bright
    pub duration: Option<u32>, // in minutes, None means infinite, 0 is burned out
    pub is_dirty: bool
}

impl ProvidesIllumination {
    pub fn new(bright_interval: u16, shadowy_interval: u16, duration: Option<u32>) -> Self {
        Self {
            illuminated_tiles: HashSet::new(),
            bright_interval,
            shadowy_interval,
            duration,
            is_dirty: true,
        }
    }
    pub fn clone_dirty(&self) -> Self {
        Self {
            illuminated_tiles: HashSet::new(),
            bright_interval: self.bright_interval,
            shadowy_interval: self.shadowy_interval,
            duration: self.duration,
            is_dirty: true,
        }
    }
}

pub fn illumination_system(
    mut light_sources: Query<(Entity, &map::MapPoint, &mut ProvidesIllumination)>,
    mut mb: ResMut<MapBuilder>
) {
    let mut changed: HashSet<Point> = HashSet::new();
    for (entity, map_point, mut provides_illumination) in light_sources.iter_mut() {
        // illuminate all the tiles within the entities' line of sight
        if provides_illumination.is_dirty {
            let bright_distance = provides_illumination.bright_interval as f32;
            let shadowy_distance = (provides_illumination.bright_interval + provides_illumination.shadowy_interval) as f32;
            let range = (provides_illumination.bright_interval + provides_illumination.shadowy_interval) / distance::TILE_SIZE_IN_FEET;
            let old_set = provides_illumination.illuminated_tiles.clone();
            provides_illumination.illuminated_tiles = field_of_view_set(map_point.0, range as i32, &mb.map);
            for x in old_set.difference(&provides_illumination.illuminated_tiles) {
                let idx = mb.map.map_idx(x.x, x.y);
                let tile = &mut mb.map.tiles[idx];
                tile.illuminated_by.remove(&entity);
                changed.insert(*x);
            }
            for x in &provides_illumination.illuminated_tiles {
                // compute the distance fom map_point.0
                // add or change illumination level
                let distance = distance_between_points(map_point.0, *x);
                let light_level = if distance <= bright_distance {
                    IlluminationLevel::Normal
                } else if distance <= shadowy_distance {
                    IlluminationLevel::Dim
                } else {
                    IlluminationLevel::None
                };
                let idx = mb.map.map_idx(x.x, x.y);
                let tile = &mut mb.map.tiles[idx];
                tile.illuminated_by.insert(entity, light_level);
                changed.insert(*x);
            }
            provides_illumination.is_dirty = false;
        }
    }
    for x in changed {
        // Compute the total illumination of each changed point
        // Change the tile color based on illumination
        let idx = mb.map.map_idx(x.x, x.y);
        let tile = &mut mb.map.tiles[idx];
        let mut normal_count = 0;
        let mut dim_count = 0;
        for (_, level) in tile.illuminated_by.clone() {
            match level {
                IlluminationLevel::Normal => {
                    normal_count = normal_count + 1;
                },
                IlluminationLevel::Dim => {
                    dim_count = dim_count + 1;
                },
                _ => {}
            }
        }
        if normal_count > 0 || dim_count > 1 {
            tile.illumination_level = IlluminationLevel::Normal;
        } else if dim_count > 0 {
            tile.illumination_level = IlluminationLevel::Dim;
        } else {
            tile.illumination_level = IlluminationLevel::None;
        }
    }
}
