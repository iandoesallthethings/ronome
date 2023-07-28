#![allow(dead_code)]
use crate::midi::Midi;
use monome::{KeyDirection, Monome, MonomeEvent};
use std::thread::sleep;
use std::time::Duration;

const REFRESH_INTERVAL: Duration = Duration::from_millis(2);

type GridHandler<Context> = fn(grid: &mut Grid<Context>);
type PixelHandler<Context> = fn(grid: &mut Grid<Context>, x: i32, y: i32);

pub struct Grid<Context = ()> {
    m: Monome,
    pub pixels: Vec<u8>,
    pub context: Context,
    pub midi: Midi,
    key_down_handler: PixelHandler<Context>,
    key_up_handler: PixelHandler<Context>,
    frame_handler: GridHandler<Context>,
}

impl Grid<()> {
    pub fn new() -> Grid<()> {
        Grid::<()>::new_with_context(())
    }
}

impl<Context> Grid<Context> {
    pub fn new_with_context<NewContext>(context: NewContext) -> Grid<NewContext> {
        Grid {
            m: Monome::new("/prefix").unwrap(),
            midi: Midi::new(),
            pixels: vec![0; 128],
            key_down_handler: |_, _, _| {},
            key_up_handler: |_, _, _| {},
            frame_handler: |_| {},
            context,
        }
    }

    // API within handlers
    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        let index = self.coordinate_to_index(x, y);

        if index > 0 && index < self.pixels.len() {
            self.pixels[index]
        } else {
            0
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, intensity: u8) {
        let index = self.coordinate_to_index(x, y);

        if (0..self.pixels.len()).contains(&index) {
            self.pixels[index] = intensity;
        }
    }
    pub fn set_row(&mut self, y: i32, intensity: u8) {
        for x in 0..16 {
            self.set_pixel(x, y, intensity);
        }
    }

    pub fn set_column(&mut self, x: i32, intensity: u8) {
        for y in 0..8 {
            self.set_pixel(x, y, intensity);
        }
    }

    pub fn set_all(&mut self, new_pixels: Vec<u8>) {
        self.pixels = new_pixels;
    }

    pub fn toggle_pixel(&mut self, x: i32, y: i32) {
        let intensity = Grid::get_pixel(&self, x, y);
        Grid::set_pixel(self, x, y, !intensity);
    }

    pub fn map_pixels(
        &mut self,
        mapper: fn(grid: &mut Grid<Context>, x: i32, y: i32, intensity: u8, index: usize) -> u8,
    ) -> Vec<u8> {
        self.pixels
            .to_owned()
            .iter()
            .enumerate()
            .map(|(index, &intensity)| {
                let (x, y) = &self.index_to_coordinate(index);

                mapper(self, *x, *y, intensity, index)
            })
            .collect::<Vec<u8>>()
    }

    pub fn clear(&mut self) {
        self.pixels = vec![0; 128];
    }

    // Builders
    pub fn on_key_down(mut self, handler: PixelHandler<Context>) -> Grid<Context> {
        self.key_down_handler = handler;
        self
    }

    pub fn on_key_up(mut self, handler: PixelHandler<Context>) -> Grid<Context> {
        self.key_up_handler = handler;
        self
    }

    pub fn on_frame(mut self, handler: GridHandler<Context>) -> Grid<Context> {
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
        self.m.poll() // Just forwarding this for now
    }

    fn handle_input(&mut self) {
        match self.poll() {
            Some(MonomeEvent::GridKey { x, y, direction }) => match direction {
                KeyDirection::Down => (self.key_down_handler)(self, x, y),
                KeyDirection::Up => (self.key_up_handler)(self, x, y),
            },
            _ => {}
        }
    }

    fn update_state(&mut self) {
        (self.frame_handler)(self);
    }

    fn draw(&mut self) {
        self.m.set_all_intensity(&self.pixels);
    }

    // Helpers
    pub fn coordinate_to_index(&self, x: i32, y: i32) -> usize {
        ((y * 16) + x) as usize
    }

    pub fn index_to_coordinate(&self, index: usize) -> (i32, i32) {
        let x = index % 16;
        let y = index / 16;

        (x as i32, y as i32)
    }
}

fn is_in_range(x: i32, y: i32) -> bool {
    x >= 0 && x <= 15 && y >= 0 && y <= 7
}
