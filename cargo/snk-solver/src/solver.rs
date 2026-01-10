use snk_grid::{
    direction::Direction,
    grid::{Color, Grid},
    snake::Snake4,
};

pub fn solve(grid: &Grid<Color>, snake: &Snake4) -> Vec<Direction> {
    vec![]
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
