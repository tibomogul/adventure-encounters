use super::MapArchitect;
use crate::prelude::*;

use super::MapBuilder;
use super::Map;
use super::MapTheme;
use super::Tile;
use super::TileType;

const STAGGER_DISTANCE: usize = 400;

pub struct DrunkardsWalkArchitect {}
impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, width: u32, height: u32, theme: MapTheme, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let map = Map::new(width, height, theme);
        let mut mb = MapBuilder {
            map,
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme
        };
        mb.fill(TileType::ThemeWall);
        let center = Point::new(width / 2, height / 2);
        let desired_floor = (width * height) / 3;
        self.drunkard(&center, rng, &mut mb);
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| (**t).tile_type == mb.theme.tile_to_render(TileType::ThemeFloor))
            .count()
            < desired_floor as usize
        {
            self.drunkard(
                &Point::new(rng.range(0, width), rng.range(0, height)),
                rng,
                &mut mb,
            );
            let dijkstra_map = DijkstraMap::new(
                width,
                height,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = Tile::new(mb.theme.tile_to_render(TileType::ThemeWall)));
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, mb: &mut MapBuilder) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;
        loop {
            let drunk_idx = mb.map.point2d_to_index(drunkard_pos);
            mb.map.tiles[drunk_idx] = Tile::new(mb.theme.tile_to_render(TileType::ThemeFloor));
            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            if !mb.map.in_bounds(drunkard_pos) {
                break;
            }
            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
