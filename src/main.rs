use std::env;
use std::io::{self, Read};
use std::fs::File;

use rust_cpu_emu::emulator::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("arguments error");
        return;
    }
    let mut program = match read_binary(&args[1]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("file read error: {:?}", e);
            return;
        },
    };
    let mut emu = Emulator::new(&mut program, 0x7C00, 0x7C00);
    emu.start();
}

fn read_binary(filename: &String) -> io::Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut buffer = vec![0; 0x200];
    f.read(&mut buffer)?;
    Ok(buffer)
}
