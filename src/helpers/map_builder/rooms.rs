use crate::prelude::*;

use super::Map;
use super::MapArchitect;
use super::MapBuilder;
use super::MapTheme;
use super::TileType;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, width: u32, height: u32, theme: MapTheme, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let map = Map::new(width, height, theme);
        let mut mb = MapBuilder{
            map,
            rooms: Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero(),
            theme,
        };
        mb.fill(TileType::ThemeWall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        mb
    }
}