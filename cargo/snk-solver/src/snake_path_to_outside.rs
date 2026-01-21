use snk_grid::{
    color::Color,
    direction::{Direction, add_direction, iter_directions, iter_neighbour},
    grid::{Grid, iter_rectangle_hull},
    grid_samples::{SampleGrid, get_grid_sample},
    point::Point,
    snake::{Snake, Snake4, snake_will_self_collide},
};
use std::collections::{BinaryHeap, HashSet};

use crate::{
    cost::Cost,
    path_to_outside_grid::{ExitDirection, create_path_to_outside},
};

#[derive(Clone, Debug)]
struct Node {
    pub snake: Snake4,
    pub cost: Cost,
    pub path: Vec<Direction>,
}

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.snake == other.snake
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub fn get_snake_path_to_outside<FnOutside, FnCost>(
    is_outside: FnOutside,
    get_walk_cost: FnCost,
    snake: &Snake4,
) -> (Vec<Direction>, Cost)
where
    FnOutside: Fn(Point) -> bool,
    FnCost: Fn(Point) -> Cost,
{
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    let mut close_list: HashSet<Snake4> = HashSet::new();

    open_list.push(Node {
        snake: snake.clone(),
        cost: Cost::zero(),
        path: Vec::new(),
    });

    while let Some(node) = open_list.pop() {
        let head = node.snake.get_head();
        if is_outside(head) {
            return (node.path, node.cost);
        }

        for dir in iter_directions() {
            if snake_will_self_collide(&node.snake, dir) {
                continue;
            }

            let snake = node.snake.clone_and_move(dir);

            if close_list.contains(&snake) {
                // need to update open_list
                println!("need to update open_list");
                continue;
            }

            let head = snake.get_head();

            let cost = node.cost + get_walk_cost(head);
            let mut path = node.path.clone();
            path.push(dir);

            open_list.push(Node { cost, path, snake });
        }

        close_list.insert(node.snake);
    }

    assert!(false, "should have terminated");
    (vec![], Cost::zero())
}

#[test]
fn it_should_get_snake_path_to_outside() {
    let grid = Grid::<_>::from(
        r#"
_########  _
_#    .  . _
_#      # _
_########  _

"#,
    );
    let pto = create_path_to_outside(&grid);
    let snake = Snake4::from_points([
        Point { x: 2, y: 2 },
        Point { x: 3, y: 2 },
        Point { x: 4, y: 2 },
        Point { x: 5, y: 2 },
    ]);

    let is_outside = |p| !pto.is_inside(p) || pto.get(p).is_outside();

    let (path, cost) = get_snake_path_to_outside(is_outside, |p| grid.get_color(p).into(), &snake);

    println!("{:?} {:?}", path, cost);
    assert!(
        cost < Cost::from(Color::Color2) * 2,
        "should have taken the smallest path"
    );
}
