use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Unit {
    Feet,
    Miles,
    Tiles
}

pub const TILE_SIZE_IN_FEET: u16 = 5;

pub fn distance_between_points(x: Point, y: Point) -> f32 {
    DistanceAlg::Pythagoras.distance2d(x, y) * TILE_SIZE_IN_FEET as f32
}