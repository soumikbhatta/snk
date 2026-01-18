use snk_grid::{
    color::Color,
    direction::iter_neighbour,
    grid::{Grid, iter_rectangle_hull},
    point::Point,
};
use std::collections::HashSet;

struct CollectionCost {
    in_cost: u32,
    out_cost: u32,
}

pub fn get_collect_cost(is_outside: F, get_cost: C) -> CollectionCost
where
    F: Fn(Point) -> bool,
    F: Fn(Point) -> u32,
{
}
