use crate::grid::Grid as GenericGrid;

type Grid = GenericGrid<Mode>;

pub fn main() {
    let mode = Mode::Toggle;

    Grid::new_with_context(mode)
        .on_key_down(press)
        .on_key_up(release)
        .run();
}

fn press(grid: &mut Grid, x: i32, y: i32) {
    if x == 0 && y == 0 {
        grid.context.switch();
        return;
    }

    match grid.context {
        Mode::Toggle => grid.toggle_pixel(x, y),
        Mode::Hold => grid.set_pixel(x, y, 255),
    };
}

fn release(grid: &mut Grid, x: i32, y: i32) {
    match grid.context {
        Mode::Hold => grid.set_pixel(x, y, 0),
        _ => {}
    };
}

#[derive(Debug)]
enum Mode {
    Toggle,
    Hold,
}

impl Mode {
    fn switch(&mut self) {
        match self {
            Mode::Toggle => *self = Mode::Hold,
            Mode::Hold => *self = Mode::Toggle,
        };

        dbg!(self);
    }
}
