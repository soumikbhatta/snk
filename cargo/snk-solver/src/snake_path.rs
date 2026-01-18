use snk_grid::{
    direction::{Direction, iter_directions},
    point::{Point, get_distance},
    snake::{Snake, Snake4, snake_will_self_collide},
};
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Debug)]
struct Node {
    pub snake: Snake4,
    pub weight: u32,
    pub heuristic: u32,
    pub parent: Option<(Box<Node>, Direction)>,
}
impl Node {
    fn f(&self) -> u32 {
        self.weight + self.heuristic
    }
}
impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f().cmp(&other.f()).reverse()
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_snake_path<F>(
    get_walk_cost: F,
    from: Snake4,
    to: Point,
    max_weight: u32,
) -> Option<Vec<Direction>>
where
    F: Fn(Point) -> u32,
{
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    let mut close_list: HashSet<Snake4> = HashSet::new();

    open_list.push(Node {
        heuristic: get_distance(from.get_head(), to) as u32,
        snake: from,
        weight: 0,
        parent: None,
    });

    while let Some(node) = open_list.pop() {
        // println!(
        //     "{:?}",
        //     open_list
        //         .clone()
        //         .into_iter()
        //         .map(|n| n.weight)
        //         .collect::<Vec<_>>()
        // );

        for dir in iter_directions() {
            if snake_will_self_collide(&node.snake, dir) {
                continue;
            }
            let snake = node.snake.clone_and_move(dir);

            if close_list.contains(&snake) {
                // need to update open_list
                continue;
            }

            let head = snake.get_head();

            if to == head {
                // unroll and return
                let mut path = vec![dir];

                let mut parent = node.parent;
                while let Some((node, dir)) = parent {
                    path.push(dir);
                    parent = node.parent;
                }

                path.reverse();
                return Some(path);
            }

            let weight = node.weight + get_walk_cost(head);
            let heuristic = get_distance(head, to) as u32;

            if weight + heuristic > max_weight {
                continue;
            }

            open_list.push(Node {
                snake,
                weight,
                heuristic,
                parent: Some((Box::new(node.clone()), dir)),
            });
        }

        close_list.insert(node.snake);
    }

    None
}

#[test]
fn it_should_found_simple_path() {
    let snake = Snake4::from_points([
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 3, y: 0 },
    ]);
    let res = get_snake_path(|_| 1, snake, Point { x: 0, y: 3 }, 9999);

    assert_eq!(
        res,
        Some(vec![
            //
            Direction::UP,
            Direction::UP,
            Direction::UP,
        ])
    )
}
