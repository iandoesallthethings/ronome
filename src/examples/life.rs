use crate::grid::Grid as GenericGrid;
use std::thread::sleep;
use std::time::Duration;

type Grid = GenericGrid<Context>;
type Control = (i32, i32);
const RUN: Control = (0, 0);
const STEP: Control = (1, 0);

struct Context {
    running: bool,
    step_interval: Duration,
}
pub fn main() {
    let context = Context {
        running: false,
        step_interval: Duration::from_millis(100),
    };

    Grid::new_with_context(context)
        .on_key_down(controls)
        .on_frame(step_if_running)
        .run();
}

fn controls(grid: &mut Grid, x: i32, y: i32) {
    match (x, y) {
        RUN => {
            grid.context.running = !grid.context.running;

            let intensity = if grid.context.running { 15 } else { 0 };
            grid.set_pixel(x, y, intensity);
        }

        STEP => {
            step(grid);
        }

        (2..=15, 0) => {
            grid.set_row(0, 0);
            let previous_setting = f64::sqrt(grid.context.step_interval.as_millis() as f64);
            grid.set_pixel(previous_setting as i32, 0, 0);

            grid.context.step_interval = Duration::from_millis(u64::pow(x as u64, 2));

            grid.set_pixel(x, 0, x as u8)
        }

        _ => {
            let current = grid.get_pixel(x, y);
            let new = match current {
                0 => 15,
                _ => 0,
            };

            grid.set_pixel(x, y, new);
        }
    }
}

fn step_if_running(grid: &mut Grid) {
    if grid.context.running {
        step(grid);
    }
}

fn play_notes_if_on(grid: &mut Grid) {
    grid.map_pixels(|grid, x, y, intensity, _| {
        if y == 0 {
            return 0;
        }

        let note = isometric_fourths(x, y);

        match intensity {
            0 => {
                grid.midi.note_off(note, 127);
            }
            1..=15 => {
                grid.midi.note_on(note, 127);
            }
            _ => {}
        };

        0
    });
}

fn isometric_fourths(x: i32, y: i32) -> u8 {
    let octave_offset = 12 * 3; // Push up to usable octaves
    let x_offset = -5; // Sliding down so c is in the bottom left
    let y_offset = (8 - y) * 5; // reverse y axis * perfect 4th

    (x + x_offset + y_offset + octave_offset) as u8
}

fn step(grid: &mut Grid) {
    let new_pixels = grid.map_pixels(conway_rules);

    grid.set_all(new_pixels);

    play_notes_if_on(grid);

    sleep(grid.context.step_interval);
}

fn conway_rules(grid: &mut Grid, x: i32, y: i32, intensity: u8, _index: usize) -> u8 {
    // First row is controls - I'll bet there's a way to refactor controls()
    // to just omit the actual control keys.
    if y == 0 {
        return intensity;
    }

    let neighbors = num_living_neighbors(grid, x, y);
    let alive = intensity > 0;

    (match (alive, neighbors) {
        (true, 0..=1) => 0,   // Starved
        (true, 4..=8) => 0,   // Crowded
        (true, _) => 15 - x,  // Safe
        (false, 3) => 15 - x, // Fertile
        (false, _) => 0,      // Barren
    } as u8)
}

fn num_living_neighbors(grid: &mut Grid, x: i32, y: i32) -> usize {
    NEIGHBOR_MASK
        .iter()
        .filter_map(|(dx, dy)| match grid.get_pixel(x + dx, y + dy) > 0 {
            true => Some(true),
            false => None,
        })
        .collect::<Vec<bool>>()
        .len()
}

const NEIGHBOR_MASK: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
