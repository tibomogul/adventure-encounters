use crate::prelude::*;

use super::MapArchitect;
use super::MapBuilder;
use super::Map;
use super::MapTheme;
use super::Tile;
use super::TileType;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, width: u32, height: u32, theme: MapTheme, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let map = Map::new(width, height, theme);
        let mut mb = MapBuilder{
            map,
            rooms : Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero(),
            theme
        };
        self.random_noise_map(rng, &mut mb);
        for _ in 0..10 {
            self.iteration(&mut mb);
        }
        let start = self.find_start(&mb);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(
        &mut self,
        rng: &mut RandomNumberGenerator,
        mb: &mut MapBuilder)
    {
        mb.map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = Tile::new(mb.theme.tile_to_render(TileType::ThemeFloor));
            } else {
                *t = Tile::new(mb.theme.tile_to_render(TileType::ThemeWall));
            }
        });
    }    
    fn count_neighbors(&self, x: i32, y: i32, mb: &MapBuilder) -> usize {
        let mut neighbors = 0;
        for iy in -1 ..= 1 {
            for ix in -1 ..= 1 {
                if !(ix==0 && iy == 0)
                {
                    let x2 = x+ix;
                    let y2 = y+iy;
                    if mb.map.in_bounds(Point { x: x2, y: y2 })
                        && mb.map.tiles[mb.map.map_idx(x2, y2)].tile_type == mb.theme.tile_to_render(TileType::ThemeWall) {
                        neighbors += 1;
                    }
                }
            }
        }
        neighbors
    }
    fn iteration(&mut self, mb: &mut MapBuilder) {
        let mut new_tiles = mb.map.tiles.clone();
        for y in 1 .. mb.map.dimensions.x -1 {
            for x in 1 .. mb.map.dimensions.y -1 {
                let neighbors = self.count_neighbors(x, y, mb);
                let idx = mb.map.map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = Tile::new(mb.theme.tile_to_render(TileType::ThemeWall));
                } else {
                    new_tiles[idx] = Tile::new(mb.theme.tile_to_render(TileType::ThemeFloor));
                }
            }
        }
        mb.map.tiles = new_tiles;
    }
    fn find_start(&self, mb: &MapBuilder) -> Point {
        let center = Point::new(mb.map.dimensions.x/2, mb.map.dimensions.y/2);
        let closest_point = mb.map.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| (**t).tile_type == mb.theme.tile_to_render(TileType::ThemeFloor))
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(
                center,
                mb.map.index_to_point2d(idx)
            )))
            .min_by(|(_, distance), (_, distance2)|
                distance.partial_cmp(&distance2).unwrap()
            )
            .map(|(idx, _)| idx)
            .unwrap();
        mb.map.index_to_point2d(closest_point)
    }
}