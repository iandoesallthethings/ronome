use crate::grid::Grid;

pub fn main() {
    Grid::new()
        .on_key_down(note_on)
        .on_key_up(note_off)
        .on_frame(draw_c)
        .run()
}

fn note_on(grid: &mut Grid, x: i32, y: i32) {
    grid.set_pixel(x, y, 15);

    let note = isometric_fourths(x, y);
    grid.midi.note_on(note, 127);
}

fn note_off(grid: &mut Grid, x: i32, y: i32) {
    grid.set_pixel(x, y, 0);

    let note = isometric_fourths(x, y);
    grid.midi.note_off(note, 127);
}

fn draw_c(grid: &mut Grid) {
    grid.map_pixels(|grid, x, y, _, _| {
        let note = isometric_fourths(x, y);

        if note % 12 == 0 {
            let intensity = (note / 12) * 2;
            grid.set_pixel(x, y, intensity);
            intensity
        } else {
            0
        }
    });
}

/* Basically Deluge 🤷‍♂️ */
fn isometric_fourths(x: i32, y: i32) -> u8 {
    let octave_offset = 12 * 3; // Push up to usable octaves
    let x_offset = -5; // Sliding down so c is in the bottom left
    let y_offset = (8 - y) * 5; // reverse y axis * perfect 4th

    (x + x_offset + y_offset + octave_offset) as u8
}
