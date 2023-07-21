#![allow(dead_code)]
use monome::{KeyDirection, Monome, MonomeEvent};
use std::{thread, time};
const REFRESH_RATE: u64 = 2;

type GridHandler = fn(grid: &mut Grid);
type PixelHandler = fn(grid: &mut Grid, x: i32, y: i32);

pub struct Grid {
    pixels: Vec<u8>,
    m: Monome,
    on_key_down: PixelHandler,
    on_key_up: PixelHandler,
    on_frame: GridHandler,
}

impl Grid {
    pub fn new(prefix: String) -> Grid {
        Grid {
            m: Monome::new(prefix).unwrap(),
            pixels: vec![0; 128],
            on_key_down: |_, _, _| {},
            on_key_up: |_, _, _| {},
            on_frame: |_| {},
        }
    }

    // API
    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        let index = Grid::coordinate_to_index(x, y);
        self.pixels[index]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, intensity: u8) {
        let index = Grid::coordinate_to_index(x, y);
        self.pixels[index] = intensity;
    }

    pub fn toggle_pixel(&mut self, x: i32, y: i32) {
        let intensity = Grid::get_pixel(&self, x, y);
        Grid::set_pixel(self, x, y, !intensity);
    }

    pub fn clear(&mut self) {
        self.pixels = vec![0; 128];
    }

    // Builders
    pub fn key_down(mut self, handler: PixelHandler) -> Grid {
        self.on_key_down = handler;
        self
    }

    pub fn key_up(mut self, handler: PixelHandler) -> Grid {
        self.on_key_up = handler;
        self
    }

    pub fn frame(mut self, handler: GridHandler) -> Grid {
        self.on_frame = handler;
        self
    }

    pub fn run(&mut self) {
        loop {
            self.handle_input();
            (self.on_frame)(self);
            self.draw();

            let refresh = time::Duration::from_millis(REFRESH_RATE);
            thread::sleep(refresh);
        }
    }

    // Helpers
    fn coordinate_to_index(x: i32, y: i32) -> usize {
        ((y * 16) + x) as usize
    }

    fn index_to_coordinate(index: usize) -> (i32, i32) {
        let x = index % 16;
        let y = index / 16;

        (x as i32, y as i32)
    }

    //  Runtime
    fn poll(&mut self) -> Option<MonomeEvent> {
        self.m.poll()
    }

    fn handle_input(&mut self) {
        match self.poll() {
            Some(MonomeEvent::GridKey { x, y, direction }) => match direction {
                KeyDirection::Down => (self.on_key_down)(self, x, y),
                KeyDirection::Up => (self.on_key_up)(self, x, y),
            },
            _ => { /* nuthin */ }
        }
    }

    fn draw(&mut self) {
        self.m.set_all_intensity(&self.pixels);
    }
}
