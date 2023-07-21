use monome::{KeyDirection, Monome, MonomeEvent};
use std::{thread, time};

fn main() {
    let mut m = Monome::new("/prefix").unwrap();

    let mut grid = vec![0; 128];

    loop {
        match m.poll() {
            Some(MonomeEvent::GridKey { x, y, direction }) => match direction {
                KeyDirection::Down => {
                    println!("Key pressed: {}x{}", x, y);

                    let index = (y as usize * 16) + x as usize;

                    grid[index] = !grid[index];
                }
                KeyDirection::Up => {
                    // println!("Key released: {}x{}", x, y);
                    // m.set(x, y, false);
                }
            },
            _ => {
                // break;
            }
        }

        m.set_all_intensity(&grid);

        let refresh = time::Duration::from_millis(5);
        thread::sleep(refresh);
    }
}
