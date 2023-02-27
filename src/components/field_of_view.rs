use std::collections::HashSet;
use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView{
    pub visible_tiles : HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool
}