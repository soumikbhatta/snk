use log::info;
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
    pub distance: u32,
    pub f: u32,
    pub path: Vec<Direction>,
}

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f.cmp(&other.f)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_snake_path<F>(
    get_walk_cost: F,
    from: &Snake4,
    to: Point,
    max_weight: u32,
) -> Option<Vec<Direction>>
where
    F: Fn(Point) -> u32,
{
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    let mut close_list: HashSet<Snake4> = HashSet::new();

    open_list.push(Node {
        distance: 0,
        snake: from.clone(),
        weight: 0,
        f: 0,
        path: Vec::new(),
    });

    open_list.push(Node {
        distance: 0,
        snake: from.clone(),
        weight: 0,
        f: 3,
        path: Vec::new(),
    });

    open_list.push(Node {
        distance: 0,
        snake: from.clone(),
        weight: 0,
        f: 1,
        path: Vec::new(),
    });
    open_list.push(Node {
        distance: 0,
        snake: from.clone(),
        weight: 0,
        f: 5,
        path: Vec::new(),
    });

    log::info!(
        "open_list {:?}",
        open_list
            .clone()
            .into_iter()
            .map(|n| n.f)
            .collect::<Vec<_>>()
    );

    let mut loop_count = 0;

    while let Some(node) = open_list.pop() {
        loop_count += 1;

        log::info!(
            "loop {} node {} {:?} open_list {:?}",
            loop_count,
            node.f,
            node.snake.get_head(),
            open_list
                .clone()
                .into_iter()
                .map(|n| n.f)
                .collect::<Vec<_>>()
        );

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
                let mut path = node.path.clone();
                path.push(dir);

                log::info!(" {:?} : {:?}", to, head);
                log::info!("found loop {} length: {}", loop_count, path.len());

                path.reverse();
                return Some(path);
            }

            let weight = node.weight + get_walk_cost(head);
            let distance = get_distance(head, to) as u32;
            log::info!(" {:?} {:?}", head, distance);

            let f = weight + distance;
            // if f > max_weight {
            //     continue;
            // }

            let mut path = node.path.clone();
            path.push(dir);
            open_list.push(Node {
                snake,
                weight,
                distance,
                f,
                path,
            });
        }

        close_list.insert(node.snake);

        log::info!(
            "open_list {:?}",
            open_list
                .clone()
                .into_iter()
                .map(|n| n.f)
                .collect::<Vec<_>>()
        );
    }

    None
}

#[test]
#[ignore]
fn it_should_found_simple_path() {
    let snake = Snake4::from_points([
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 3, y: 0 },
    ]);
    let res = get_snake_path(|_| 1, &snake, Point { x: 0, y: 3 }, 9999);

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
