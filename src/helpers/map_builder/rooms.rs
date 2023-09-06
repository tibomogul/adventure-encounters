use crate::prelude::*;

use super::Map;
use super::MapArchitect;
use super::MapBuilder;
use super::MapTheme;
use super::TileType;
use crate::helpers::prelude::Tile;

const NUM_ROOMS: usize = 20;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, width: u32, height: u32, theme: MapTheme, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let map = Map::new(width, height, theme);
        let mut mb = MapBuilder{
            map,
            rooms: Vec::new(),
            monster_spawns : Vec::new(),
            entity_spawns: Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero(),
            theme,
        };
        mb.fill(TileType::ThemeWall);
        self.build_random_rooms(&mut mb, rng);
        self.build_corridors(&mut mb, rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        mb
    }
}

impl RoomsArchitect {
    fn build_random_rooms(&mut self, mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        while mb.rooms.len() < NUM_ROOMS {
            let room = bracket_geometry::prelude::Rect::with_size(
                rng.range(1, mb.map.dimensions.x - 10),
                rng.range(1, mb.map.dimensions.y - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in mb.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < mb.map.dimensions.x && p.y > 0 && p.y < mb.map.dimensions.y {
                        let idx = mb.map.map_idx(p.x, p.y);
                        mb.map.tiles[idx] = Tile::new(mb.theme.tile_to_render(TileType::ThemeFloor));
                    }
                });
                mb.rooms.push(room)
            }
        }
    }
    fn apply_vertical_tunnel(&mut self, mb: &mut MapBuilder, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = mb.map.try_idx(Point::new(x, y)) {
                mb.map.tiles[idx as usize] = Tile::new(mb.theme.tile_to_render(TileType::ThemeFloor));
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, mb: &mut MapBuilder, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = mb.map.try_idx(Point::new(x, y)) {
                mb.map.tiles[idx as usize] = Tile::new(mb.theme.tile_to_render(TileType::ThemeFloor));
            }
        }
    }
    fn build_corridors(&mut self, mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        let mut rooms = mb.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(mb, prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(mb, prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(mb, prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(mb, prev.x, new.x, new.y);
            }
        }
    }
}