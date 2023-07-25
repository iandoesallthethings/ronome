use midi_control::{
    Channel, KeyEvent,
    MidiMessage::{self, NoteOff, NoteOn},
};
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::io::{stdin, stdout, Write};

pub struct Midi {
    out: MidiOutputConnection,
}

impl Midi {
    pub fn new() -> Midi {
        Midi {
            out: Midi::connect(),
        }
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        let event = NoteOn(
            Channel::Ch1,
            KeyEvent {
                key: note,
                value: velocity,
            },
        );

        self.send(event);
    }

    pub fn note_off(&mut self, note: u8, velocity: u8) {
        let event = NoteOff(
            Channel::Ch1,
            KeyEvent {
                key: note,
                value: velocity,
            },
        );

        self.send(event);
    }

    fn send(&mut self, event: MidiMessage) {
        let buffer: Vec<u8> = event.into();
        self.out.send(buffer.as_slice()).unwrap();
    }

    fn connect() -> MidiOutputConnection {
        let midi_out = MidiOutput::new("My Test Output").unwrap();

        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let out_port: &MidiOutputPort = match out_ports.len() {
            0 => panic!("no output port found"),
            1 => {
                println!(
                    "Choosing the only available output port: {}",
                    midi_out.port_name(&out_ports[0]).unwrap()
                );
                &out_ports[0]
            }
            _ => {
                println!("\nAvailable output ports:");
                for (i, p) in out_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("Please select output port: ");
                stdout().flush().unwrap();

                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                out_ports
                    .get(input.trim().parse::<usize>().unwrap())
                    .ok_or("invalid output port selected")
                    .unwrap()
            }
        };

        println!("\nOpening connection");

        midi_out.connect(out_port, "midir-test").unwrap()
    }
}
