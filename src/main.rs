use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::Read};

struct Rom {
	buf: Vec<u8>,
	program_counter: usize,
}

impl Rom {
	fn new(buf: Vec<u8>) -> Self {
		Rom {
			buf,
			program_counter: 0,
		}
	}
	fn next(&mut self) -> u8 {
		self.program_counter += 1;
		if self.program_counter - 1 >= self.buf.len() {
			std::process::exit(0)
		}
		self.buf[self.program_counter - 1]
	}
}

fn main() -> io::Result<()> {
	let mut file = File::open("fantaset.hex")?;
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)?;

	let mut rom = Rom::new(buffer);

	let mut current_store = 0;

	let mut stores = HashMap::new();

	stores.insert(0, HashMap::<u8, u16>::new());

	loop {
		match rom.next() {
			0xe1 => {
				match stores.get_mut(&current_store) {
					Some(store) => {
						store.insert(
							rom.next(),
							u16::from_be_bytes(
								[rom.next(), rom.next()]
							)
						);
					}
					None => {
						stores.insert(current_store, HashMap::new());
						stores.get_mut(&current_store).unwrap().insert(
							// Fully cognizant that this is repeated code.
							// Too lazy to put it in a func. 
							rom.next(), 
							u16::from_be_bytes(
								[rom.next(), rom.next()]
							)
						);
					}
				};
			},
			0xd3 => {
				let mut to_display = Vec::new();
				loop {
					match rom.next() {
						0x00 => { match rom.next() as char {
								'n' => to_display.push(0x000a),
								't' => to_display.push(0x000b),
								_ => {}
							}
						}
						0xd3 => break,
						any => to_display.push(
							*stores.get(&current_store).unwrap()
							.get(&any).unwrap()
						)
					}
				}
				println!("{}", String::from_utf16(&to_display).unwrap())
			}
			any => panic!("Unimplemented: {:x}", any),
		}
	}

	Ok(())
}
