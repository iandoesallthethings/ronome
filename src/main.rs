use std::env::args;

mod flash;
mod grid;
mod keyboard;
mod life;
mod midi;
mod modes;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "flash" => flash::main(),
            "modes" => modes::main(),
            "life" => life::main(),
            "keyboard" => keyboard::main(),
            _ => println!("Demo not found."),
        }
    } else {
        keyboard::main();
    }
}
