use crate::prelude::{*, map_builder::MapBuilder, map::{FogOfWarMapLayer, SeenLevel}, distance::distance_between_points};
use std::collections::HashSet;

use super::illumination::IlluminationLevel;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct FieldOfView{
    pub visible_tiles : HashSet<Point>,
    pub dark_vision: Option<u16>,
    pub dim_vision: Option<u16>,
    pub normal_vision: u16,
    pub is_dirty: bool
}

impl FieldOfView {
    pub fn new(normal_vision: u16, dim_vision: Option<u16>, dark_vision: Option<u16>) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            // true_sight, https://www.dandwiki.com/wiki/5e_SRD:Truesight
            // blind_sight, https://www.dandwiki.com/wiki/5e_SRD:Blindsight does not need light
            // tremor_sight, https://www.dandwiki.com/wiki/5e_SRD:Creatures#Tremorsense maybe can be incorpoarated into blind_sight
            dark_vision, // https://www.dandwiki.com/wiki/5e_SRD:Darkvision Many creatures in fantasy gaming worlds, especially those that dwell underground, have darkvision. Within a specified range, a creature with darkvision can see in darkness as if the darkness were dim light, so areas of darkness are only lightly obscured as far as that creature is concerned. However, the creature can't discern color in darkness, only shades of gray.
            dim_vision,
            normal_vision,
            is_dirty: true,
        }
    }
    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            dark_vision: self.dark_vision,
            dim_vision: self.dim_vision,
            normal_vision: self.normal_vision,
            is_dirty: true,
        }
    }
}

// TODO: FOV is affected by conditions from source tile. The range for vision is dependent on environment. On a clear day you an see up 
// to 2.9 miles to the horizon. Your abiliy to perceive things will vary. A huge object like a tower might be visible, but a small human
// might not be. As a guide, maybe we can take the effective range of ranged weapons as the maximum where you can decern a human-sized target.
pub fn field_of_view_system(
    mut fovs: Query<(Entity, &map::MapPoint, &mut FieldOfView)>,
    mut mb: ResMut<MapBuilder>,
    mut fog_of_war: Query<(&mut TileStorage, &FogOfWarMapLayer)>,
    mut commands: Commands
) {
    let mut changed: HashSet<Point> = HashSet::new();
    // TODO: based on environment, determine the range of sight/perception: bright day, foggy marsh, etc
    // TODO: based on conditions: temporary blindness, confusion, poison 
    // Process individual FOVs and determine tiles seen
    for (entity, map_point, mut fov) in fovs.iter_mut() {
        // illuminate all the tiles within the entities' line of sight
        if fov.is_dirty {
            let range = fov.normal_vision / distance::TILE_SIZE_IN_FEET; // TODO: This is the value to be addressed
            let old_set:HashSet<Point> = fov.visible_tiles.clone();
            fov.visible_tiles = field_of_view_set(map_point.0, range as i32, &mb.map);
            for x in old_set.difference(&fov.visible_tiles) {
                // tiles going out of sight
                // remove player from sighters
                let idx = mb.map.map_idx(x.x, x.y);
                let tile = &mut mb.map.tiles[idx];
                tile.seen_by.remove(&entity);
                changed.insert(*x);
            }
            for x in &fov.visible_tiles {
                // compute distance
                // add or change sight level
                let idx = mb.map.map_idx(x.x, x.y);
                let tile = &mut mb.map.tiles[idx];
                // TODO: Consider obscurity level on the tile, e.g. fog, smoke, magical effects
                // This factors in the illumination level
                let light_level = tile.illumination_level;
                let distance = distance_between_points(map_point.0, *x);
                let seen_level: SeenLevel = match light_level {
                    IlluminationLevel::None => {
                        if let Some(dark_vision) = fov.dark_vision {
                            if distance <= dark_vision as f32 {
                                SeenLevel::Shadowy
                            } else {
                                SeenLevel::Darkness
                            }
                        } else {
                            SeenLevel::Darkness
                        }
                    },
                    IlluminationLevel::Dim => {
                        if let Some(dark_vision) = fov.dark_vision {
                            if distance <= dark_vision as f32 {
                                SeenLevel::Bright
                            } else {
                                SeenLevel::Shadowy
                            }
                        } else {
                            SeenLevel::Shadowy
                        }
                    },
                    IlluminationLevel::Normal => SeenLevel::Bright,
                };
                tile.seen_by.insert(entity, seen_level);
                changed.insert(*x);
            }
            fov.is_dirty = false;
        }
    }
    // Combine effects of different FOVs on tiles seen
    let (tile_storage, _) = fog_of_war.single_mut();
    for x in changed {
        let tile_pos = mb.map.to_bevy_ecs_tilemap(x.x, x.y);
        let idx = mb.map.map_idx(x.x, x.y);
        let tile = &mut mb.map.tiles[idx];
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            let mut bright_count = 0;
            let mut shadowy_count = 0;
            let mut dark_count = 0;
            for (_, level) in tile.seen_by.clone() {
                match level {
                    SeenLevel::Bright => {
                        bright_count = bright_count + 1;
                    },
                    SeenLevel::Shadowy => {
                        shadowy_count = shadowy_count + 1;
                    },
                    SeenLevel::Darkness => {
                        dark_count = dark_count + 1;
                    },
                    SeenLevel::None => {}
                }
            }
            match tile.seen_level {
                SeenLevel::Bright => {},
                SeenLevel::Shadowy => {
                    if bright_count > 0 {
                        tile.seen_level = SeenLevel::Bright;
                    }
                },
                SeenLevel::Darkness => {
                    if bright_count > 0 {
                        tile.seen_level = SeenLevel::Bright;
                    } else if shadowy_count > 0 {
                        tile.seen_level = SeenLevel::Shadowy;
                    }
                },
                SeenLevel::None => {
                    if bright_count > 0 {
                        tile.seen_level = SeenLevel::Bright;
                    } else if shadowy_count > 0 {
                        tile.seen_level = SeenLevel::Shadowy;
                    } else if dark_count > 0 {
                        tile.seen_level = SeenLevel::Darkness;
                    }
                }
            }
            // Update Fog of War layer --> put this in a separate system, together with visibility of entities
            match tile.seen_level {
                SeenLevel::Bright => { commands.entity(tile_entity).insert(TileColor(Color::rgba(0.0, 0.0, 0.0, 0.0))); },
                SeenLevel::Shadowy => { commands.entity(tile_entity).insert(TileColor(Color::rgba(0.0, 0.0, 0.0, 0.70))); },
                SeenLevel::Darkness => { commands.entity(tile_entity).insert(TileColor(Color::rgba(0.0, 0.0, 0.0, 0.90))); },
                SeenLevel::None => { commands.entity(tile_entity).insert(TileColor(Color::rgba(0.0, 0.0, 0.0, 1.0))); },
            }
        }
    }
}

