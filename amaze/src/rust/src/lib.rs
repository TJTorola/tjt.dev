mod utils;

use std::convert::TryInto;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel(u8, u8, u8, u8);

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel(r, g, b, 255)
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.0 = r;
        self.1 = g;
        self.2 = b;
    }
}

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let data = vec![Pixel::new(0, 0, 0); (width * height * 4).try_into().unwrap()];

        Image {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> *const Pixel {
        self.data.as_ptr()
    }

    fn get_idx(&self, x: u32, y: u32) -> usize {
        (x * self.width + y) as usize
    }

    pub fn paint_region(&mut self, from: (u32, u32), to: (u32, u32), color: (u8, u8, u8)) {
        let (x1, y1) = from;
        let (x2, y2) = to;
        let (r, g, b) = color;

        for x in x1..x2 {
            for y in y1..y2 {
                let idx = self.get_idx(x, y);
                self.data[idx].set(r, g, b);
            }
        }
    }
}

#[wasm_bindgen]
pub struct Maze {
    cell_size: u32,
    wall_size: u32,
    cols: u32,
    rows: u32,
    image: Image,
}

#[wasm_bindgen]
impl Maze {
    pub fn new(cell_size: u32, max_width: u32, max_height: u32) -> Maze {
        let wall_size = cell_size / 10;
        let full_size = wall_size + cell_size;

        let rows = (max_height + wall_size) / full_size;
        let cols = (max_width + wall_size) / full_size;
        let height = (rows * full_size) - wall_size;
        let width = (cols * full_size) - wall_size;

        let image = Image::new(width, height);

        Maze {
            cell_size,
            wall_size,
            cols,
            rows,
            image,
        }
    }

    pub fn image_data(&self) -> *const Pixel {
        self.image.data()
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    fn get_region(&self, row: u32, col: u32) -> ((u32, u32), (u32, u32)) {
        let full_size = self.cell_size + self.wall_size;
        (
            (row * full_size, col * full_size),
            ((row * full_size) + self.cell_size, (col * full_size) + self.cell_size),
        )
    }

    pub fn tick(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let (to, from) = self.get_region(row, col);
                self.image.paint_region(to, from, (255, 255, 255));
            }
        }
    }
}
























