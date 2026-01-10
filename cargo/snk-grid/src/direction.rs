use crate::point::{Point, add};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::DOWN,
    Direction::LEFT,
    Direction::RIGHT,
];

impl Direction {
    pub fn to_point(&self) -> Point {
        match self {
            Direction::UP => Point { x: 0, y: 1 },
            Direction::DOWN => Point { x: 0, y: -1 },
            Direction::LEFT => Point { x: -1, y: 0 },
            Direction::RIGHT => Point { x: 1, y: 0 },
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        DIRECTIONS.iter().map(|dir| dir.clone())
    }
}

pub fn add_direction(a: Point, dir: Direction) -> Point {
    add(a, dir.to_point())
}

pub fn iter_directions() -> impl Iterator<Item = Direction> {
    DIRECTIONS.iter().map(|dir| dir.clone())
}

pub fn iter_neighbour(p: Point) -> impl Iterator<Item = Point> {
    iter_directions().map(move |dir| add_direction(p, dir))
}

#[test]
fn it_should_iter_direction() {
    let directions: Vec<_> = iter_directions().collect();

    assert_eq!(
        directions,
        vec![
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ]
    );
}

#[test]
fn it_should_iter_direction_point() {
    let directions: Vec<_> = iter_directions().map(|d| d.to_point()).collect();

    assert_eq!(
        directions,
        vec![
            Point { x: 0, y: 1 },
            Point { x: 0, y: -1 },
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 }
        ]
    );
}
