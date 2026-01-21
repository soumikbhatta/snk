use snk_grid::{
    color::Color,
    direction::{Direction, add_direction, iter_directions},
    grid::{Grid, iter_rectangle_hull},
    grid_ascii::grid_to_ascii_transformed,
    grid_samples::{SampleGrid, get_grid_sample},
    point::Point,
};
use std::collections::HashSet;

use crate::cost::Cost;

#[derive(Copy, Clone)]
pub struct ExitDirection {
    pub cost: Cost,
    pub exit_direction: Direction,
}
impl ExitDirection {
    pub fn is_outside(&self) -> bool {
        self.cost.is_free()
    }
}

//
// cost_to_outside : for each cell return the minimal cost ( = sum of dot, with greater color costing more ) to get outside
pub fn create_path_to_outside(grid: &Grid<Color>) -> Grid<ExitDirection> {
    let mut path_to_outside = Grid::<ExitDirection>::create_with_value(
        grid.width,
        grid.height,
        ExitDirection {
            cost: Cost::max(),
            exit_direction: Direction::UP,
        },
    );

    let mut changed: HashSet<Point> =
        iter_rectangle_hull(grid.width as i8 + 2, grid.height as i8 + 2)
            .map(|mut p| {
                p.x -= 1;
                p.y -= 1;
                p
            })
            .collect();

    while let Some(p) = {
        let next = changed.iter().next();
        next.map(|p| *p)
    } {
        changed.remove(&p);

        let cost: Cost = if path_to_outside.is_inside(p) {
            path_to_outside.get(p).cost
        } else {
            Cost::zero()
        };

        for dir in iter_directions() {
            let p = add_direction(p, dir);
            if path_to_outside.is_inside(p) {
                let new_cost = cost + grid.get(p).into();

                let c = path_to_outside.get_mut(p);

                if new_cost < c.cost {
                    c.cost = new_cost;
                    c.exit_direction = dir.get_opposite();
                    changed.insert(p);
                }
            }
        }
    }

    path_to_outside
}

#[test]
fn it_should_compute_the_cost_to_outside() {
    let grid = Grid::<_>::from(
        r#"
_...._
_.  ._
_.. ._
_.   _
"#,
    );
    let pto = create_path_to_outside(&grid);

    assert_eq!(
        grid_to_ascii_transformed(&pto, |c| {
            if c.cost.is_free() {
                c.cost.0.to_string()
            } else {
                "#".to_string()
            }
        }),
        r#"
1####1
1#43#1
1##2#1
1#1111
"#
        .trim(),
    );
}
