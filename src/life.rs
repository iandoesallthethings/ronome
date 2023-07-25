use crate::grid::Grid as GenericGrid;

type Grid = GenericGrid<bool>;

pub fn main() {
    let running = false;

    Grid::new_with_context(running)
        .on_key_down(controls)
        .on_frame(step_if_running)
        .run();
}

fn controls(grid: &mut Grid, x: i32, y: i32) {
    match (x, y) {
        //  Run
        (0, 0) => {
            grid.context = !grid.context;
            let intensity = if grid.context { 15 } else { 0 };
            grid.set_pixel(x, y, intensity);
        }
        // Step
        (1, 0) => {
            step(grid);
        }
        _ => {
            grid.toggle_pixel(x, y);
        }
    }
}

fn step_if_running(grid: &mut Grid) {
    if grid.context {
        step(grid);
    }
}

fn step(grid: &mut Grid) {
    let new_pixels = grid.map_pixels(conway_rules);

    grid.set_all(new_pixels);
}

enum State {
    Alive = 15,
    Dead = 0,
}

fn conway_rules(grid: &mut Grid, x: i32, y: i32, intensity: u8, _index: usize) -> u8 {
    // First row is controls
    if x == 0 {
        return intensity;
    }

    let neighbors = num_living_neighbors(grid, x, y);
    let alive = intensity > 0;

    (match (alive, neighbors) {
        (true, 0..=1) => State::Dead, // Starved
        (true, 4..=8) => State::Dead, // Crowded
        (true, _) => State::Alive,    // Safe
        (false, 3) => State::Alive,   // Fertile
        (false, _) => State::Dead,    // Barren
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
