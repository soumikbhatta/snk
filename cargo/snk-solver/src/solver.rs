use std::collections::HashMap;

use snk_grid::{
    color::Color,
    direction::Direction,
    grid::{Grid, iter_rectangle_fill},
    point::Point,
    snake::Snake4,
};

use crate::{
    best_tunnel::{Tunnel, get_best_tunnel_to_collect_point},
    cost::Cost,
    exit_grid::ExitGrid,
};

pub const COLORS: [Color; 4] = [Color::Color1, Color::Color2, Color::Color3, Color::Color4];

pub fn solve(color_grid: &Grid<Color>, snake: &Snake4) -> Vec<Direction> {
    let exit_grid = ExitGrid::create_from_grid_color(color_grid);

    let snake = snake.clone();
    let path = Vec::new();

    for color in COLORS.into_iter() {
        let to_collect = iter_rectangle_fill(color_grid.width, color_grid.height)
            .filter(|p| color_grid.get_color(*p) == color)
            .map(|p| {
                let tunnel = get_best_tunnel_to_collect_point(&color_grid, &exit_grid, p);
                let score: Cost = tunnel.in_cost.set_empty_to_zero() * 2
                    + tunnel.out_cost.set_empty_to_zero() * 3;
                let cost_to_outside = exit_grid.get_cost_to_outside(p);
                (p, tunnel, score, cost_to_outside)
            });

        let (free_to_collect, mut rest): (Vec<_>, Vec<_>) =
            to_collect.partition(|(_, _, global_cost, _)| global_cost.is_free());

        // todo!("collect free points");

        rest.sort_by(|a, b| a.2.cmp(&b.2));

        // take the first
        // then we need to recompute the tunnel is the cost_to_outside changed
    }

    path
}

#[cfg(test)]
mod tests {
    use crate::fitness::get_solution_fitness;

    use super::*;
    use snk_grid::{
        grid_samples::{SAMPLEGRIDS, SampleGrid, get_grid_sample},
        point::Point,
        snake::Snake4,
    };

    #[test]
    #[ignore]
    fn it_should_found_solution_for_one_dot() {
        let grid = get_grid_sample(SampleGrid::OneDot);
        let snake = Snake4::from_points([
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 },
            Point { x: 2, y: -1 },
            Point { x: 3, y: -1 },
        ]);
        let solution = solve(&grid, &snake);

        let fitness = get_solution_fitness(&grid, snake, solution);

        assert_eq!(
            fitness.remaining_color_count, 0,
            "Remaining color count should be 0"
        );
        assert_eq!(
            fitness.stack_fitness, 0,
            "Stack should be perfectly orderer"
        );
    }

    #[test]
    #[ignore]
    fn it_should_found_solution_for_labyrinth() {
        let grid = get_grid_sample(SampleGrid::Labyrinth);
        let snake = Snake4::from_points([
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 },
            Point { x: 2, y: -1 },
            Point { x: 3, y: -1 },
        ]);
        let solution = solve(&grid, &snake);

        let fitness = get_solution_fitness(&grid, snake, solution);

        assert_eq!(
            fitness.remaining_color_count, 0,
            "Remaining color count should be 0"
        );
        assert_eq!(
            fitness.stack_fitness, 0,
            "Stack should be perfectly orderer"
        );
    }

    #[test]
    #[ignore]
    fn it_should_found_solution_with_no_remaining_colors() {
        for s in SAMPLEGRIDS {
            let grid = get_grid_sample(s);
            let snake = Snake4::from_points([
                Point { x: 0, y: -1 },
                Point { x: 1, y: -1 },
                Point { x: 2, y: -1 },
                Point { x: 3, y: -1 },
            ]);
            let solution = solve(&grid, &snake);

            let fitness = get_solution_fitness(&grid, snake, solution);

            assert_eq!(
                fitness.remaining_color_count, 0,
                "Remaining color count should be 0"
            );
        }
    }
}
