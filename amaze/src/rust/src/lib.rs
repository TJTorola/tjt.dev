mod utils;

extern crate web_sys;
extern crate js_sys;
extern crate im;

use std::convert::TryInto;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

type Coord = (usize, usize);
type Region = (Coord, Coord);
type Color = (u8, u8, u8);
type Change = (Coord, Color);
type Diff = Vec<Coord>;
type Map = im::HashMap<Coord, Color>;
type Step = (Diff, Map);

fn clamp(i: usize, low: usize, high: usize) -> usize {
    if i < low {
        low
    } else if i >= high {
        high - 1
    } else {
        i
    }
}

fn randNum(to: usize) -> usize {
    (js_sys::Math::random() * to as f64) as usize
}

fn generateRandSteps(rows: usize, cols: usize) -> Process {
  let mut process = Process::new(None); 
  for _ in 0..255 {
      let changes = (0..5).map(|_| {(
          (randNum(cols), randNum(rows)),
          (randNum(256) as u8, randNum(256) as u8, randNum(256) as u8),
      )}).collect();

      process.push(changes);
  }
  process
}

fn generateTestStep(coord: Coord) -> Process {
    let mut process = Process::new(None);
    process.push(vec![(coord, (255, 255, 255))]);
    process
}

pub struct Process {
  steps: Vec<(Diff, Map)>,
}

impl Process {
    pub fn new(init: Option<Map>) -> Process {
        let init = init.unwrap_or(im::HashMap::new());
        Process {
            steps: vec![(vec![], init)]
        }
    }

    pub fn get_top_map(&self) -> Map {
        let top = self.steps.get(self.steps.len() - 1).unwrap();
        top.1.clone()
    }

    pub fn get_map(&self, step_idx: usize) -> Option<Map> {
        match self.steps.get(step_idx) {
            Some(step) => Some(step.1.clone()),
            None => None,
        }
    }

    pub fn push(&mut self, changes: Vec<Change>) {
        let diff = changes.iter().map(|(coord, _)| {
            *coord
        }).collect();

        let map = changes.iter().fold(self.get_top_map(), |acc, (coord, color)| {
            acc.update(*coord, *color)
        });

        self.steps.push((diff, map));
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn get_diff(&self, from: usize, to: Option<usize>) -> Diff {
        if to.is_none() {
            match self.steps.get(from) {
                Some(step) => step.0.clone(),
                None => vec![],
            }
        } else {
            let to = to.unwrap();
            let low = std::cmp::min(to, from) + 1;
            let high = std::cmp::max(to, from);

            (low..=high).fold(vec![], |acc, idx| {
                match self.steps.get(idx) {
                    Some(step) => [&acc[..], &step.0[..]].concat(),
                    None => acc,
                }
            })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel(u8, u8, u8, u8);

impl Pixel {
    pub fn new(color: Color) -> Pixel {
        let (r, g, b) = color;
        Pixel(r, g, b, 255)
    }

    pub fn set(&mut self, color: Color) {
        let (r, g, b) = color;
        self.0 = r;
        self.1 = g;
        self.2 = b;
    }
}

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let data = vec![Pixel::new((0, 0, 0)); (width * height * 4).try_into().unwrap()];

        Image {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> *const Pixel {
        self.data.as_ptr()
    }

    fn get_idx(&self, coord: Coord) -> usize {
        let (x, y) = coord;
        (y * self.width + x) as usize
    }

    pub fn paint_region(&mut self, region: Region, color: Color) {
        let ((x1, y1), (x2, y2)) = region;

        for x in x1..x2 {
            for y in y1..y2 {
                let idx = self.get_idx((x, y));
                self.data[idx].set(color);
            }
        }
    }
}

#[wasm_bindgen]
pub struct Maze {
    cell_size: usize,
    wall_size: usize,
    cols: usize,
    rows: usize,
    image: Image,
    step_idx: usize,
    process: Process,
}

#[wasm_bindgen]
impl Maze {
    pub fn new(cell_size: usize, wall_size: usize, max_width: usize, max_height: usize) -> Maze {
        let full_size = wall_size + cell_size;

        let cell_rows = (max_height + wall_size) / full_size;
        let cell_cols = (max_width + wall_size) / full_size;
        let height = (cell_rows * full_size) - wall_size;
        let width = (cell_cols * full_size) - wall_size;

        let rows = (cell_rows * 2) - 1;
        let cols = (cell_cols * 2) - 1;

        let image = Image::new(width, height);

        Maze {
            cell_size,
            wall_size,
            cols,
            rows,
            image,
            step_idx: 0,
            process: generateRandSteps(rows, cols),
        }
    }

    pub fn image_data(&self) -> *const Pixel {
        self.image.data()
    }

    pub fn width(&self) -> usize {
        self.image.width()
    }

    pub fn height(&self) -> usize {
        self.image.height()
    }

    pub fn step_count(&self) -> usize {
        self.process.len()
    }

    fn get_region(&self, coord: Coord) -> Region {
        let full_size = self.cell_size + self.wall_size;
        let (x, y) = coord;

        let x1 = (full_size * (x / 2)) + (self.cell_size * (x % 2));
        let y1 = (full_size * (y / 2)) + (self.cell_size * (y % 2));

        let height = if (y % 2 == 0) { self.cell_size } else { self.wall_size };
        let width = if (x % 2 == 0) { self.cell_size } else { self.wall_size };

        let x2 = x1 + width;
        let y2 = y1 + height;

        ((x1, y1), (x2, y2))
    }

    pub fn set_step(&mut self, new_step_idx: usize) {
        let map = self.process.get_map(new_step_idx).unwrap();
        let diff = self.process.get_diff(self.step_idx, Some(new_step_idx));

        for coord in diff.iter() {
            let color = match map.get(coord) {
                Some(c) => c.clone(),
                None => (0, 0, 0),
            };
            let region = self.get_region(*coord);
            self.image.paint_region(region, color);
        }
        self.step_idx = new_step_idx;
    }
}
