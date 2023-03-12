use std::collections::{HashMap, HashSet};

use crate::prelude::*;

use super::map::Map;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridPoint {
    pub point: Point,
    cost: u32,
    g: u32,
    neighbors: Vec<Point>,
    via: Option<Point>,
}

impl GridPoint {
    fn new(x: u16, y: u16, cost: u32) -> Self {
        Self {
            point: Point::new(x, y),
            cost,
            g: 0,
            neighbors: Vec::new(),
            via: None,
        }
    }
}

pub struct RangeFinder {}

impl RangeFinder {
    pub fn compute_grid(anchor: Point, range: u32, map: Map) -> HashMap<Point, GridPoint> {
        let mut grid: HashMap<Point, GridPoint> = HashMap::new();
        let mut open_set: HashSet<Point> = HashSet::new();
        let mut closed_set: HashSet<Point> = HashSet::new();

        grid.insert(anchor, GridPoint::new(anchor.x as u16, anchor.y as u16, 5));
        open_set.insert(anchor);

        while !open_set.is_empty() {
            // get the first with the lowest cost
            // on start this will just be the origin tile, with
            // cost of zero.
            let mut some_lowest_cost_grid_point: Option<&GridPoint> = None;
            let mut lowest_cost_point: Option<Point> = None;
            for point in &open_set {
                let grid_point = grid.get(&point).unwrap();

                if some_lowest_cost_grid_point.is_none() || some_lowest_cost_grid_point.unwrap().cost < grid_point.cost {
                    some_lowest_cost_grid_point = Some(grid_point);
                    lowest_cost_point = Some(grid_point.point.clone());
                }
            }

            //remove current from openSet
            let lowest_cost_grid_point = some_lowest_cost_grid_point.unwrap();
            let current_point = lowest_cost_grid_point.point;
            open_set.remove(&current_point);
            //add current to closedSet
            closed_set.insert(current_point);
            let current_cost = lowest_cost_grid_point.g;
            let mut neighbors = Vec::new();
            if current_point.x < map.dimensions.x - 1 {
                neighbors
                    .push(current_point + Point::new(1, 0));
            }
            if current_point.x > 0 {
                neighbors
                    .push(current_point + Point::new(-1, 0));
            }
            if current_point.y < map.dimensions.y - 1 {
                neighbors
                    .push(current_point + Point::new(0, 1));
            }
            if current_point.y > 0 {
                neighbors
                    .push(current_point + Point::new(0, -1));
            }
    
            // Get all the neighbours, then iterate
            for i in 0..neighbors.len() {
                // Check if the neighbour has been processed
                if !(closed_set.contains(&neighbors[i])) {
                    if !grid.contains_key(&neighbors[i]) {
                        let idx = map.map_idx(neighbors[i].x, neighbors[i].y);
                        let tile = &map.tiles[idx];
                        grid
                            .insert(neighbors[i], GridPoint::new(neighbors[i].x as u16, neighbors[i].y as u16, tile.terrain_cost as u32));
                    }
                    let mut neighbor = grid.get_mut(&neighbors[i]).unwrap();

                    // The cost of coming here from the current tile
                    // is the total to the current tile plus
                    // the cost of entering this tile
                    let possible_g = current_cost + neighbor.cost;

                    // If the cost is already greater than the range
                    // do not consider this tile anymore
                    if possible_g > range {
                        continue;
                    }

                    // If not yet in consideration, all to open set
                    // If already in consideration, and the cost from
                    // current is higher than current total, skip
                    if !(open_set.contains(&neighbor.point)) {
                        open_set.insert(neighbor.point);
                    } else if possible_g >= neighbor.g {
                        continue;
                    }

                    // Otherwise update current cost, and choose current
                    // as preferred
                    neighbor.g = possible_g;
                    neighbor.via = lowest_cost_point;
                }
            }
        }
        grid
    }

    pub fn get_grid(grid: HashMap<Point, GridPoint>) -> Vec<Point> {
        grid.clone()
            .iter()
            .map(|(_, x)| x )
            .filter(|x| x.via.is_some())
            .map(|x| x.point)
            .collect()
    }

    pub fn get_path_to(grid: HashMap<Point, GridPoint>, point: Point) -> Vec<Point> {
        let mut path = Vec::new();
        match grid.get(&point) {
            None => path,
            Some(mut grid_point) => {
                while let Some(via) = grid_point.via {
                    path.push(via);
                    match grid.get(&via) {
                        None => break,
                        Some(next) => {
                            grid_point = next;
                        }
                    };
                }
                path
            }
        }
    }
}
