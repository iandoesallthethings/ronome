use crate::grid::Grid;

const FLASH: u8 = 254;
const DIM: u8 = 2;

pub fn main() {
    Grid::new()
        .on_key_down(toggle_pressed)
        // .on_key_up(key_up)
        .on_frame(fade)
        .run();
}

fn toggle_pressed(grid: &mut Grid, x: i32, y: i32) {
    grid.set_pixel(x, y, FLASH)
}

fn fade(grid: &mut Grid) {
    for (index, intensity) in grid.pixels.to_owned().iter().enumerate() {
        let (x, y) = grid.index_to_coordinate(index);

        if intensity > &0 {
            grid.set_pixel(x, y, intensity - DIM);
        }
    }
}
