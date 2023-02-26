use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    ThemeFloor, // should map to something else
    ThemeWall, // Should map to somethine else
    ThemeExit, // should map to something else
    FloorSandStone0, //51
    WallShoals2, //233
    GatewaysLairEnter,// 144
    FloorDirt0, // 35
    WallTreesMangrove1, // 176
    GatewaysDepthsEnter, // 137
}

impl TileType {
    pub fn to_texture_index(&self) -> u8 {
        match self {
            Self::FloorSandStone0 => 51,
            Self::WallShoals2 => 233,
            Self::GatewaysLairEnter => 144,
            Self::FloorDirt0 => 35,
            Self::WallTreesMangrove1 => 176,
            Self::GatewaysDepthsEnter => 137,
            _ => panic!("not mapped")
        }
    }
}

#[derive(Component)]
pub struct TileIsOpaque;

#[derive(Component)]
pub struct TerrainCost(u8);

#[derive(Copy, Clone, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
    pub is_opaque: bool,
    pub terrain_cost: u8
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        match tile_type {
            TileType::FloorSandStone0 | TileType::FloorDirt0 => Self {
                tile_type,
                is_opaque: false,
                terrain_cost: 5
            },
            TileType::WallShoals2 | TileType::WallTreesMangrove1 => Self {
                tile_type,
                is_opaque: true,
                terrain_cost: 0
            },
            TileType::GatewaysLairEnter | TileType::GatewaysDepthsEnter => Self {
                tile_type,
                is_opaque: true,
                terrain_cost: 0
            },
            _ => panic!("Unknown TileType")
        }
    }
    pub fn insert_components(&self, mut commands: Commands, entity: Entity) {
        if self.is_opaque {
            commands.entity(entity).insert(TileIsOpaque);
        }
        if self.terrain_cost > 0 {
            commands.entity(entity).insert(TerrainCost(self.terrain_cost));
        }
    }
}