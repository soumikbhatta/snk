use snk_grid::{
    direction::iter_neighbour,
    grid::{Color, Grid, iter_rectangle_hull},
    point::Point,
};
use std::{collections::HashSet, usize};

pub fn update_cost_to_outside(
    cost_to_outside: &mut Grid<usize>,
    grid: &Grid<Color>,
    mut changed: HashSet<Point>,
) -> () {
    while let Some(p) = {
        let next = changed.iter().next();
        next.map(|p| *p)
    } {
        changed.remove(&p);

        let c = cost_to_outside.get(p);

        // propagate the change to its neightbourn
        for pn in iter_neighbour(p) {
            if cost_to_outside.is_inside(pn) {
                let new_cost = c + (grid.get(pn) as usize) * 100;
                if new_cost < cost_to_outside.get(pn) {
                    cost_to_outside.set(pn, new_cost);
                    changed.insert(pn);
                }
            }
        }
    }
}

//
// cost_to_outside : for each cell return the minimal cost ( = sum of dot, with greater color costing more ) to get outside
pub fn create_cost_to_outside(grid: &Grid<Color>) -> Grid<usize> {
    let mut cost_to_outside = Grid::<usize>::create_with_value(grid.width, grid.height, usize::MAX);

    let mut changed = HashSet::<Point>::new();

    for p in iter_rectangle_hull(grid.width as i8, grid.height as i8) {
        let cost = (grid.get(p) as usize) * 100;
        cost_to_outside.set(p, cost);
        changed.insert(p);
    }

    update_cost_to_outside(&mut cost_to_outside, grid, changed);

    cost_to_outside
}

#[test]
fn it_should_compute_the_cost_to_outside() {
    let grid = Grid::<_>::from(
        r#"
_.. _
_. ._
_..._
"#,
    );
    let cost = create_cost_to_outside(&grid);

    assert_eq!(
        cost.to_string(),
        r#"
01100
01110
01110
"#
        .trim(),
    );
}
