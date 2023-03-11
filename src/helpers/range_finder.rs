use std::collections::HashMap;

use crate::prelude::*;

use super::map::Map;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridPoint<'a> {
    pub point: Point,
    cost: u32,
    g: u32,
    neighbors: std::vec::Vec<&'a Box<GridPoint<'a>>>,
    via: Option<&'a Box<GridPoint<'a>>>,
}

impl GridPoint<'_> {
    fn new(x: u16, y: u16, cost: u32) -> Box<Self> {
        Box::new(Self {
            point: Point::new(x, y),
            cost,
            g: 0,
            neighbors: Vec::new(),
            via: None,
        })
    }
}

pub struct RangeFinder<'a> {
    pub anchor: Point,
    map: Map,
    grid: HashMap<Point, Box<GridPoint<'a>>>,
    open_set: Vec<&'a Box<GridPoint<'a>>>,
    closed_set: Vec<&'a Box<GridPoint<'a>>>,
}

impl<'a> RangeFinder<'a> {
    pub fn new(x: u16, y: u16, map: Map) -> Self {
        Self {
            anchor: Point::new(x, y),
            map,
            grid: HashMap::new(),
            open_set: Vec::new(),
            closed_set: Vec::new(),
        }
    }

    fn find_point(&self, point: Point) -> Option<&Box<GridPoint>> {
        self.grid.get(&point)
    }

    fn find_or_create_point(&mut self, point: Point) -> &mut Box<GridPoint> {
        if !self.grid.contains_key(&point) {
            self.grid
                .insert(point, GridPoint::new(point.x as u16, point.y as u16, 5));
        }
        return self.grid.get_mut(&point).unwrap();
    }

    fn consider_neighbors(&mut self, grid_point: &'a mut Box<GridPoint>) {
        self.update_neighbors(grid_point);
        self.open_set.push(grid_point);
    }

    fn update_neighbors(&self, grid_point: &'a mut Box<GridPoint<'a>>) {
        if grid_point.point.x < self.map.dimensions.x - 1 {
            grid_point
                .neighbors
                .push(self.find_or_create_point(grid_point.point + Point::new(1, 0)));
        }
        if grid_point.point.x > 0 {
            grid_point
                .neighbors
                .push(self.find_or_create_point(grid_point.point + Point::new(-1, 0)));
        }
        if grid_point.point.y < self.map.dimensions.y - 1 {
            grid_point
                .neighbors
                .push(self.find_or_create_point(grid_point.point + Point::new(0, 1)));
        }
        if grid_point.point.y > 0 {
            grid_point
                .neighbors
                .push(self.find_or_create_point(grid_point.point + Point::new(0, -1)));
        }
    }

    pub fn compute_grid(&mut self, range: u32) {
        self.grid = HashMap::new();
        self.open_set = Vec::new();
        self.closed_set = Vec::new();

        let mut start = self.find_or_create_point(self.anchor);

        self.consider_neighbors(start);

        while self.open_set.len() > 0 {
            // get the first with the lowest cost
            // on start this will just be the origin tile, with
            // cost of zero.
            let mut index_of_lowest = 0;
            for i in 0..self.open_set.len() {
                if self.open_set[i].cost < self.open_set[index_of_lowest].cost {
                    index_of_lowest = i;
                }
            }

            //remove current from openSet
            let current = self.open_set.swap_remove(index_of_lowest);
            let current_cost = current.g;
            let neighbors = current.neighbors;

            // Get all the neighbours, then iterate
            for i in 0..neighbors.len() {
                let mut neighbor = self.find_or_create_point(neighbors[i].point);

                // Check if the neighbour has been processed
                if !(self.closed_set.contains(neighbor)) {
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
                    if !(self.open_set.contains(&neighbor)) {
                        self.consider_neighbors(neighbor);
                    } else if possible_g >= neighbor.g {
                        continue;
                    }

                    // Otherwise update current cost, and choose current
                    // as preferred
                    neighbor.g = possible_g;
                    neighbor.via = Some(current);
                }
            }
            //add current to closedSet
            self.closed_set.push(current);
        }
    }

    pub fn get_grid(&self) -> Vec<Point> {
        self.closed_set
            .into_iter()
            .filter(|x| x.via.is_some())
            .map(|x| x.point)
            .collect()
    }

    pub fn get_path_to(&self, point: Point) -> Option<Vec<Point>> {
        if let Some(mut map_point) = self.find_point(point) {
            let mut path = Vec::new();
            path.push(map_point.point);
            while let Some(via) = map_point.via {
                path.push(via.point);
                map_point = via;
            }
            return Some(path); // check order
        }
        return None;
    }
}
