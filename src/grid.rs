#![allow(dead_code)]
use monome::{KeyDirection, Monome, MonomeEvent};
use std::thread::sleep;
use std::time::Duration;

const REFRESH_INTERVAL: Duration = Duration::from_millis(5);

type GridHandler = fn(grid: &mut Grid);
type PixelHandler = fn(grid: &mut Grid, x: i32, y: i32);

pub struct Grid {
    m: Monome,
    pub pixels: Vec<u8>,
    key_down_handler: PixelHandler,
    key_up_handler: PixelHandler,
    frame_handler: GridHandler,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            m: Monome::new("/prefix").unwrap(),
            pixels: vec![0; 128],
            key_down_handler: |_, _, _| {},
            key_up_handler: |_, _, _| {},
            frame_handler: |_| {},
        }
    }

    // API
    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        let index = Grid::coordinate_to_index(x, y);
        self.pixels[index]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, intensity: u8) {
        let index = Grid::coordinate_to_index(x, y);

        if index > self.pixels.len() - 1 {
            return;
        }

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
    pub fn on_key_down(mut self, handler: PixelHandler) -> Grid {
        self.key_down_handler = handler;
        self
    }

    pub fn on_key_up(mut self, handler: PixelHandler) -> Grid {
        self.key_up_handler = handler;
        self
    }

    pub fn on_frame(mut self, handler: GridHandler) -> Grid {
        self.frame_handler = handler;
        self
    }

    // pub fn with_color(mut self) -> Grid {
    //     self
    // }

    pub fn run(&mut self) {
        loop {
            self.handle_input();

            self.update_state();

            self.draw();

            sleep(REFRESH_INTERVAL);
        }
    }

    //  Runtime
    fn poll(&mut self) -> Option<MonomeEvent> {
        self.m.poll()
    }

    fn handle_input(&mut self) {
        match self.poll() {
            Some(MonomeEvent::GridKey { x, y, direction }) => match direction {
                KeyDirection::Down => (self.key_down_handler)(self, x, y),
                KeyDirection::Up => (self.key_up_handler)(self, x, y),
            },
            _ => { /* nuthin */ }
        }
    }

    fn update_state(&mut self) {
        (self.frame_handler)(self);
    }

    fn draw(&mut self) {
        self.m.set_all_intensity(&self.pixels);
    }

    // Helpers
    pub fn coordinate_to_index(x: i32, y: i32) -> usize {
        ((y * 16) + x) as usize
    }

    pub fn index_to_coordinate(index: usize) -> (i32, i32) {
        let x = index % 16;
        let y = index / 16;

        (x as i32, y as i32)
    }
}
