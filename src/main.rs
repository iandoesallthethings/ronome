mod grid;
use grid::Grid;

fn main() {
    Grid::new("/prefix".to_string())
        .key_up(key_up)
        .key_down(key_down)
        .frame(frame)
        .run();
}

fn key_down(grid: &mut Grid, x: i32, y: i32) {
    println!("Key pressed: {}x{}", x, y);
    grid.toggle_pixel(x, y);
}

fn key_up(_grid: &mut Grid, x: i32, y: i32) {
    println!("Key released: {}x{}", x, y);
}

fn frame(_grid: &mut Grid) {
    // println!("Frame");
}
