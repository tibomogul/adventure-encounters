use std::fs::File;

use ron::de::from_reader;

use serde::Deserialize;

use crate::prelude::{*, tiles::TileType};
use crate::helpers::prelude::Map;

use super::{themes::MapTheme, MapBuilder};

#[derive(Clone, Deserialize, Debug)]
pub struct CustomMap {
    pub width: i32,
    pub height: i32,
    pub theme: MapTheme,
    pub player_start: (i32, i32),
    pub amulet_start: (i32, i32),
    pub tiles: String
}

pub struct CustomFileBuilder {}

impl CustomFileBuilder {
    pub fn create_map_builder(file_name: String) -> MapBuilder {
        let path = format!("resources/{}.ron", file_name);
        let file = File::open(path).expect("Failed opening file");
        let custom_map: CustomMap = from_reader(file).expect("Unable to load templates");
        let map = Map::new(custom_map.width as u32, custom_map.height as u32, custom_map.theme);
        let tiles: String = custom_map.tiles.chars().filter(|c| !c.is_whitespace()).collect();
        let mut entity_spawns = Vec::new();
        for (i, c) in tiles.chars().enumerate() {
            // TODO iterate over chars
            let point = map.index_to_point2d(i);
            match c {
                'C' => entity_spawns.push((point, c)),
                _ => {}
            }
        }
        let mut mb = MapBuilder{
            map,
            rooms: Vec::new(),
            monster_spawns : Vec::new(),
            entity_spawns: entity_spawns,
            player_start : Point::new(custom_map.player_start.0, custom_map.player_start.1),
            amulet_start : Point::new(custom_map.amulet_start.0, custom_map.amulet_start.1),
            theme: custom_map.theme
        };
        mb.fill(TileType::ThemeFloor);
        // mb.player_start = mb.rooms[0].center();
        // mb.amulet_start = mb.find_most_distant();
        // for room in mb.rooms.iter().skip(1) {
        //     mb.monster_spawns.push(room.center());
        // }
        mb
    }
}