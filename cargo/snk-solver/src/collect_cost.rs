use snk_grid::{
    color::Color,
    direction::{add_direction, iter_neighbour},
    grid::{Grid, iter_rectangle_hull},
    point::Point,
};
use std::collections::HashSet;

use crate::{cost::Cost, path_to_outside_grid::ExitDirection};

pub struct CollectionCost {
    in_cost: Cost,
    out_cost: Cost,
}

// pub fn get_collect_cost_(is_outside: F, get_cost: C) -> CollectionCost
// where
//     F: Fn(Point) -> bool,
//     F: Fn(Point) -> u32,
// {
// }

pub fn get_collect_cost(
    grid: &Grid<Color>,
    exit_grid: &Grid<ExitDirection>,
    dot: Point,
) -> CollectionCost {
    let in_cost = exit_grid.get(dot).cost;

    let mut grid = grid.clone();

    let mut p = dot;
    loop {
        let e = exit_grid.get(dot);
        if e.cost.is_free() {
            break;
        }
        grid.set(p, Color::Empty);
        p = add_direction(p, e.exit_direction);
    }

    // TODO compute a probable initial snake

    // TODO from that snake, seak the outside without self-colliding

    CollectionCost {
        in_cost,
        out_cost: Cost::zero(),
    }
}
