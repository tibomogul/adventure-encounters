use serde::Deserialize;

use crate::tiles::*;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub enum MapTheme {
    ForestTheme,
    DungeonTheme,
}

impl MapTheme {
    pub fn tile_to_render(&self, tile_type: TileType) -> TileType {
        match self {
            MapTheme::DungeonTheme => {
                match tile_type {
                    TileType::ThemeFloor => TileType::FloorSandStone0,
                    TileType::ThemeWall => TileType::WallShoals2,
                    TileType::ThemeExit => TileType::GatewaysLairEnter,
                    _ => panic!("Unknown TileType")
                }
            },
            MapTheme::ForestTheme => {
                match tile_type {
                    TileType::ThemeFloor => TileType::FloorDirt0,
                    TileType::ThemeWall => TileType::WallTreesMangrove1,
                    TileType::ThemeExit => TileType::GatewaysDepthsEnter,
                    _ => panic!("Unknown TileType")
                }
            }
        }
    }
}

