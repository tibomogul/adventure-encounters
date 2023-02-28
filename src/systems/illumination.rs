use crate::prelude::{*, map_builder::MapBuilder};
use std::collections::HashSet;

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
    mb: Res<MapBuilder>
) {
    for (_entity, map_point, mut provides_illumination) in light_sources.iter_mut() {
        // illuminate all the tiles within the entities' line of sight
        if provides_illumination.is_dirty {
            let tiles = (provides_illumination.bright_interval + provides_illumination.shadowy_interval) / distance::TILE_SIZE_IN_FEET;
            provides_illumination.illuminated_tiles = field_of_view_set(map_point.0, tiles as i32, &mb.map);
            provides_illumination.is_dirty = false;
        }
    }
}
