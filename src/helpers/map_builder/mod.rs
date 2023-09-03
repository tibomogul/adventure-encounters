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

pub trait MapArchitect {
    fn new(&mut self, width: u32, height: u32, theme: MapTheme, rng: &mut RandomNumberGenerator) -> MapBuilder;
}


#[derive(Resource)]
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<bracket_geometry::prelude::Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme : MapTheme
}

impl MapBuilder {
    pub fn new_random(width: u32, height: u32, rng: &mut RandomNumberGenerator) -> Self {
        let architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };
        let theme = match rng.range(0, 2) {
            0 => MapTheme::DungeonTheme,
            _ => MapTheme::ForestTheme
        };
        Self::new(architect, theme, width, height, rng)
    }
    pub fn new(mut architect: Box<dyn MapArchitect>, theme: MapTheme, width: u32, height: u32, rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = architect.new(width, height, theme, rng);
        apply_prefab(&mut mb, rng);
        mb.theme = theme;
        mb
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
