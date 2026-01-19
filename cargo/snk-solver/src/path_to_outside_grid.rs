use snk_grid::{
    color::Color,
    direction::{Direction, add_direction, iter_directions},
    grid::{Grid, iter_rectangle_hull},
    point::Point,
};
use std::collections::HashSet;

#[derive(Copy, Clone)]
pub struct ExitCell {
    cost: u32,
    exit_direction: Direction,
}
impl ToString for ExitCell {
    fn to_string(&self) -> String {
        if self.cost == 0 {
            "o".to_string()
        } else {
            self.exit_direction.to_string()
        }
    }
}

//
// cost_to_outside : for each cell return the minimal cost ( = sum of dot, with greater color costing more ) to get outside
pub fn create_path_to_outside(grid: &Grid<Color>) -> Grid<ExitCell> {
    let mut path_to_outside = Grid::<ExitCell>::create_with_value(
        grid.width,
        grid.height,
        ExitCell {
            cost: u32::MAX,
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

        let cost = if path_to_outside.is_inside(p) {
            path_to_outside.get(p).cost
        } else {
            0
        };

        for dir in iter_directions() {
            let p = add_direction(p, dir);
            if path_to_outside.is_inside(p) {
                let new_cost = cost + grid.get(p).cost();

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
_....
_. ..
_....
_....
"#,
    );
    let pto = create_path_to_outside(&grid);

    assert_eq!(
        pto.to_string(),
        r#"
o←↑↑↑
o←←→→
o←↓→→
o←↓↓→
"#
        .trim(),
    );
}
