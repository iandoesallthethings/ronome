# Ronome
Dipping a technicolor claw into the wonderful, buttery vat of C++ that is the Monome.

This is just a light wrapper around `monome-rs` that attempts to simplify reasoning 
about the grid and add support for common use cases like MIDI. I intend to also expand 
it to include pixel-by-pixel color for NeoTrellis along with modified firmware to support 
it (at least for the chip I have 😬).

### Getting Started
The main module `Grid` uses builders to pass in event handlers. 

```rust
Grid::new()
	.on_key_down(play_note)
	.on_key_up(stop_note)
	.on_frame(draw_animation)
	.run();
```

Your event handlers will look like this:
```rust
fn play_note(&mut grid, x: u8, y: u8) {
	// do something
}
```

`on_frame()` is just for updating the state in arbitrary ways. It just takes `&mut grid`, runs after inputs, and before drawing.

### API
`Grid` exposes a useful little set of methods for interacting with the grid as well as some basic midi functions like note on and off. For example:

```rust 
fn play_note(&mut grid, x: u8, y: u8) {
	grid.set_pixel(x, y, 15);

	grid.midi.note_on(my_note_map(x, y), 127);
}
```

### Context
There's also a way to pass in context if you need to pass external state to your handlers.

```rust
fn main() {
	let running = false

	Grid::new_with_context(running)
		.on_key_down(start)
		.on_frame(draw)
		.run();
}

// Anything you pass in will be available as grid.context in your handlers:
fn start(&mut grid, x: u8, y: u8 ) {
	grid.context = true;
}
```

For better naming, you might pass in a struct. See `life.rs` and `modes.rs` for examples.

### Hardware
* NeoTrellis
* ItsyBitsy M0 (subject to change)




### Software
* [Firmware](https://github.com/iandoesallthethings/neotrellis-grid-paletted)
* [monome-rs](https://github.com/padenot/monome-rs)

### Upgrades
- [ ] Make midi better (i.e. accept clock or notes)
- [ ] Extend serialOSC to pass color and intensity separately
- [ ] handle frame rate better than a naive call to sleep every frame lmao


![PXL_20230721_140529760](https://github.com/iandoesallthethings/ronome/assets/15148313/a525a87d-ff13-4e28-8094-180f992cb27f)
