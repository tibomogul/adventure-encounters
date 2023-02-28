use crate::prelude::{*, map_builder::MapBuilder};
use std::collections::HashSet;

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
            dark_vision,
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

pub fn field_of_view_system(
    mut fovs: Query<(Entity, &map::MapPoint, &mut FieldOfView)>,
    mb: Res<MapBuilder>
) {
    for (_entity, map_point, mut fov) in fovs.iter_mut() {
        // illuminate all the tiles within the entities' line of sight
        if fov.is_dirty {
            let tiles = fov.normal_vision / distance::TILE_SIZE_IN_FEET;
            fov.visible_tiles = field_of_view_set(map_point.0, tiles as i32, &mb.map);
            fov.is_dirty = false;
        }
    }
}
