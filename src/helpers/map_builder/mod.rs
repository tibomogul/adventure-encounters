use crate::prelude::*;

pub mod automata;
pub mod drunkard;
// mod empty;
pub mod prefab;
pub mod rooms;
pub mod themes;

// use empty::EmptyArchitect;
use automata::CellularAutomataArchitect;
use drunkard::DrunkardsWalkArchitect;
use prefab::apply_prefab;
use rooms::RoomsArchitect;
use themes::*;

use crate::map::*;
use crate::tiles::*;

trait MapArchitect {
    fn new(&mut self, width: u32, height: u32, theme: MapTheme, rng: &mut RandomNumberGenerator) -> MapBuilder;
}


const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<bracket_geometry::prelude::Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme : MapTheme
}

impl MapBuilder {
    pub fn new(width: u32, height: u32, rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };
        let theme = match rng.range(0, 2) {
            0 => MapTheme::DungeonTheme,
            _ => MapTheme::ForestTheme
        };
        let mut mb = architect.new(width, height, theme, rng);
        apply_prefab(&mut mb, rng);
        mb.theme = theme;
        mb
    }
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = bracket_geometry::prelude::Rect::with_size(
                rng.range(1, self.map.dimensions.x - 10),
                rng.range(1, self.map.dimensions.y - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < self.map.dimensions.x && p.y > 0 && p.y < self.map.dimensions.y {
                        let idx = self.map.map_idx(p.x, p.y);
                        self.map.tiles[idx] = Tile::new(self.theme.tile_to_render(TileType::ThemeFloor));
                    }
                });
                self.rooms.push(room)
            }
        }
    }
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = Tile::new(self.theme.tile_to_render(TileType::ThemeFloor));
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = Tile::new(self.theme.tile_to_render(TileType::ThemeFloor));
            }
        }
    }
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = Tile::new(self.theme.tile_to_render(tile)));
    }
    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            self.map.dimensions.x,
            self.map.dimensions.y,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );
        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }
    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                (**t).tile_type == self.theme.tile_to_render(TileType::ThemeFloor)
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
}
