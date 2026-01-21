use snk_grid::{
    color::Color,
    direction::{Direction, add_direction, iter_neighbour},
    grid::{Grid, iter_rectangle_hull},
    grid_samples::{SampleGrid, get_grid_sample},
    point::Point,
    snake::Snake4,
};
use std::collections::HashSet;

use crate::{
    cost::Cost,
    path_to_outside_grid::{ExitDirection, create_path_to_outside},
};

pub struct Tunnel {
    path: Vec<Point>,
    in_cost: Cost,
    out_cost: Cost,
}

// pub fn get_collect_cost_(is_outside: F, get_cost: C) -> CollectionCost
// where
//     F: Fn(Point) -> bool,
//     F: Fn(Point) -> u32,
// {
// }

pub fn get_collect_cost(grid: &Grid<Color>, exit_grid: &Grid<ExitDirection>, dot: Point) -> Tunnel {
    let in_cost = exit_grid.get(dot).cost;

    let mut grid = grid.clone();

    let mut path_out_1 = Vec::new();

    let mut p = dot;

    loop {
        path_out_1.push(p);

        let (dir, outside) = if exit_grid.is_inside(p) {
            let e = exit_grid.get(p);
            grid.set(p, Color::Empty);
            (e.exit_direction, e.is_outside())
        } else {
            println!("{:?}", get_outside_direction(&grid, p));
            (get_outside_direction(&grid, p), true)
        };

        if path_out_1.len() >= 6 && outside {
            break;
        }

        p += dir.into();
    }

    println!("path_out_1 {:?}", path_out_1);

    let path = path_out_1
        .iter()
        .take_while(|p| exit_grid.is_inside(**p) && !exit_grid.get(**p).is_outside())
        .collect::<Vec<_>>();

    println!("path {:?}", path);

    let mut snake = Snake4::from_points(
        path_out_1[0..4]
            .try_into()
            .expect("snake should be 4 points"),
    );

    println!("{:?}", snake);

    // TODO compute a probable initial snake

    // TODO from that snake, seak the outside without self-colliding

    Tunnel {
        path: path_out_1,
        in_cost,
        out_cost: Cost::zero(),
    }
}

fn get_outside_direction<T: Copy>(grid: &Grid<T>, p: Point) -> Direction {
    if p.x < 0 {
        Direction::LEFT
    } else if p.y < 0 {
        Direction::UP
    } else if (p.x as u8) >= grid.width {
        Direction::RIGHT
    } else if (p.y as u8) >= grid.height {
        Direction::DOWN
    } else {
        assert!(false, "fail here");
        Direction::DOWN
    }
}

#[test]
#[ignore]
fn it_should_compute_the_tunnel_for_small_cave() {
    let grid = get_grid_sample(SampleGrid::OneSmallCave);
    let pto = create_path_to_outside(&grid);

    get_collect_cost(&grid, &pto, Point { x: 4, y: 2 });

    assert!(false)
}
