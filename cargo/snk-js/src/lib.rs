use js_sys;
use log::info;
use snk_grid::{
    color::Color, direction::Direction, grid::Grid, grid_samples::SampleGrid, point::Point,
    snake::Snake4,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn log() {
    init_panic_hook();
    console_log::init_with_level(log::Level::Debug).unwrap();

    log::info!("It works!");
}

#[wasm_bindgen]
pub fn get_snake_path(grid: IColorGrid, snake: Vec<IPoint>, to: IPoint) -> Option<Vec<IPoint>> {
    let grid = snk_grid::grid::Grid::from(grid);
    let res = snk_solver::snake_path::get_snake_path(
        |p| grid.get_color(p).cost(),
        Snake4::from_points(
            snake
                .into_iter()
                .map(|p| Point::from(p))
                .collect::<Vec<_>>()
                .try_into()
                .expect("snake should be 4 points"),
        ),
        to.into(),
        99999,
    );

    res.map(|d| d.into_iter().map(|d| IPoint::from(d.to_point())).collect())
}

// #[wasm_bindgen]
// pub fn solve(grid: IColorGrid) -> js_sys::Uint8Array {
//     todo!("todo");
//     let res = snk_solver::solver::solve(snk_grid::grid::Grid::from(grid));
//     js_sys::Uint8Array::from(
//         &(res
//             .iter()
//             .map(|u| match *u {
//                 Direction::UP => 1,
//                 Direction::DOWN => 2,
//                 Direction::LEFT => 3,
//                 Direction::RIGHT => 4,
//             })
//             .collect::<Vec<u8>>())[..],
//     )
// }

#[wasm_bindgen]
pub fn get_grid_sample(sample_name: String) -> IColorGrid {
    snk_grid::grid_samples::get_grid_sample(match &sample_name[..] {
        "empty" => SampleGrid::Empty,
        "labyrinth" => SampleGrid::Labyrinth,
        "caves" => SampleGrid::Caves,
        "realistic" => SampleGrid::Realistic,
        "one-dot" => SampleGrid::OneDot,
        "solid-block" => SampleGrid::SolidBlock,
        _ => panic!("unknown sample"),
    })
    .into()
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct IColorGrid {
    pub width: u8,
    pub height: u8,
    cells: Vec<Color>,
}

#[wasm_bindgen]
impl IColorGrid {
    pub fn create(width: u8, height: u8, data: js_sys::Uint8Array) -> IColorGrid {
        if (width as usize) * (height as usize) != (data.length() as usize) {
            panic!(
                "grid {}x{} should have {} elements, found {}",
                width,
                height,
                (width as usize) * (height as usize),
                data.length()
            );
        }

        let cells: Vec<Color> = data.to_vec().iter().map(|u| Color::from(*u)).collect();

        IColorGrid {
            width,
            height,
            cells,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn cells(&self) -> js_sys::Uint8Array {
        let o: Vec<u8> = self.cells.iter().map(|u| *u as u8).collect();
        js_sys::Uint8Array::from(&o[..])
    }
}

impl From<IColorGrid> for snk_grid::grid::Grid<Color> {
    fn from(value: IColorGrid) -> Self {
        Self {
            width: value.width,
            height: value.height,
            cells: value.cells,
        }
    }
}
impl From<snk_grid::grid::Grid<Color>> for IColorGrid {
    fn from(value: snk_grid::grid::Grid<Color>) -> Self {
        Self {
            width: value.width,
            height: value.height,
            cells: value.cells,
        }
    }
}

#[wasm_bindgen]
pub struct IPoint {
    pub x: i8,
    pub y: i8,
}
#[wasm_bindgen]
impl IPoint {
    pub fn create(x: i8, y: i8) -> IPoint {
        IPoint { x, y }
    }
}
impl From<snk_grid::point::Point> for IPoint {
    fn from(value: snk_grid::point::Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<&snk_grid::point::Point> for IPoint {
    fn from(value: &snk_grid::point::Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<IPoint> for snk_grid::point::Point {
    fn from(value: IPoint) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<&IPoint> for snk_grid::point::Point {
    fn from(value: &IPoint) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
