# Ronome
Dipping a technicolor claw into the wonderful, buttery vat of C++ that is the Monome.

This is just a light wrapper around `monome-rs` that attempts to simplify reasoning 
about the grid and add support for common use cases like MIDI. I intend to also expand 
it to include pixel-by-pixel color for NeoTrellis along with modified firmware to support 
it (at least for the chip I have 😬).

### Getting Started
The main module operates on a builder pattern. 
```rust
Grid::new()
	.on_key_down()
	.on_key_up()
	.on_frame()
	.run();
```

Event handlers look like this:
```rust
fn play_note(&mut grid, x: u8, y: u8) {
	// do something
}
```

`on_frame()` is just for updating the state in arbitrary ways. It just takes `&mut grid`, runs after inputs, and before drawing.

### API
`Grid` exposes useful 
```rust 

```



### Hardware
* NeoTrellis
* ItsyBitsy M0 (subject to change)

### Software
* [Firmware](https://github.com/iandoesallthethings/neotrellis-grid-paletted)
* [monome-rs](https://github.com/padenot/monome-rs)

![PXL_20230721_140529760](https://github.com/iandoesallthethings/ronome/assets/15148313/a525a87d-ff13-4e28-8094-180f992cb27f)
