use crate::prelude::*;

use tiles::Tile;
use tiles::TileType;
use crate::map_builder::themes::MapTheme;

#[derive(Component)]
pub struct FogOfWarMapLayer;

#[derive(Component)]
pub struct ObjectsMapLayer;

#[derive(Component)]
pub struct FloorMapLayer;

#[derive(Copy, Clone, PartialEq)]
pub enum SeenType {
    Bright,
    Shadowy,
    Darkness
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct MapPoint(pub Point);

impl MapPoint {
    pub fn new(value: Point) -> Self {
        Self {
            0: value.clone()
        }
    }
}

pub struct Map {
    pub dimensions: Point,
    pub tiles: Vec<Tile>,
    pub revealed_tiles: Vec<SeenType>
}

impl Map {
    pub fn new(width: u32, height: u32, theme: MapTheme) -> Self {
        let num_tiles = width * height;
        let mut tiles = Vec::with_capacity(num_tiles as usize);
        for i in 0..num_tiles {
            tiles.push(Tile::new(theme.tile_to_render(TileType::ThemeFloor)));
        }
        Self {
            dimensions: Point { x: width as i32, y: height as i32 },
            tiles: tiles,
            revealed_tiles: vec![SeenType::Darkness; num_tiles as usize]
        }
    }
    pub fn map_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.dimensions.x) + x) as usize
    }
    pub fn try_idx(&self, point : Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(self.map_idx(point.x, point.y))
        }
    }
    pub fn to_bevy_ecs_tilemap(&self, x: i32, y:i32) -> TilePos {
        TilePos {
            x: x as u32,
            y: (self.dimensions.y - y - 1) as u32,
        }
    }
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < self.dimensions.x
            && point.y >= 0 && point.y < self.dimensions.y
    }
    pub fn can_enter_tile(&self, point : Point) -> bool {
        self.in_bounds(point) && 
            !self.tiles[self.map_idx(point.x, point.y)].is_opaque
        
    }

}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.dimensions.x, self.dimensions.y)
    }
    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize].is_opaque
    }
    fn get_available_exits(&self, idx: usize)
        -> SmallVec<[(usize, f32); 10]>
    {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        exits
    }
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(
                self.index_to_point2d(idx1),
                self.index_to_point2d(idx2)
            )
    }
}