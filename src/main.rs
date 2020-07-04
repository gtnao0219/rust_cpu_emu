use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 1024 * 1024;
// const EAX: usize = 0;
// const ECX: usize = 1;
// const EDX: usize = 2;
// const EBX: usize = 3;
const ESP: usize = 4;
// const EBP: usize = 5;
// const ESI: usize = 6;
// const EDI: usize = 7;
const REGISTERS_COUNT: usize = 8;
const INSTRUCTIONS_COUNT: usize = 0x100;

fn main() {
    let mut emu = Emulator::new(MEMORY_SIZE, 0x7C00, 0x7C00);
    let f = File::open("binary").expect("file not found");
    let mut handle = f.take(0x200);
    handle.read(&mut emu.memory[0x7C00..]).expect("file read error");

    let instructions = init_instructions();
    while (emu.eip as usize) < MEMORY_SIZE {
        let code = get_code8(&emu, 0);
        instructions[code as usize](&mut emu);
        if emu.eip == 0x00 {
            println!("need of program.");
            break;
        }
    }

    println!("{:?}", emu.registers);
}

#[derive(Debug)]
struct Emulator {
    registers: [u32; REGISTERS_COUNT],
    elags: u32,
    memory: Vec<u8>,
    eip: u32,
}

impl Emulator {
    pub fn new(size: usize, eip: u32, esp: u32) -> Emulator {
        let mut emu = Emulator{
            registers: [0; REGISTERS_COUNT],
            elags: 0,
            memory: vec![0; size],
            eip: eip,
        };
        emu.registers[ESP] = esp;
        emu
    }
}

type Instruction = fn(&mut Emulator) -> ();
type Instructions = [Instruction; INSTRUCTIONS_COUNT];
fn noop(_emu: &mut Emulator) -> () {
}
fn get_code8(emu: &Emulator, index: usize) -> u8 {
    emu.memory[(emu.eip as usize) + index] as u8
}
fn get_sign_code8(emu: &Emulator, index: usize) -> i8 {
    get_code8(&emu, index) as i8
}
fn get_code32(emu: &Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret |= (get_code8(emu, index + i) as u32) << (i * 8)
    }
    ret
}
fn get_sign_code32(emu: &Emulator, index: usize) -> i32 {
    get_code32(&emu, index) as i32
}
fn mov_r32_imm32(emu: &mut Emulator) -> () {
    let reg = get_code8(emu, 0) - 0xB8;
    let value = get_code32(emu, 1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}
fn short_jump(emu: &mut Emulator) -> () {
    let diff = get_sign_code8(&emu, 1);
    emu.eip = ((emu.eip as i32) + (diff as i32) + 2) as u32;
}
fn near_jump(emu: &mut Emulator) -> () {
    let diff = get_sign_code32(&emu, 1);
    emu.eip = ((emu.eip as i32) + (diff as i32) + 5) as u32;
}
fn init_instructions() -> Instructions {
    let mut instructions: Instructions = [noop; INSTRUCTIONS_COUNT];
    for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
    }
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
    instructions
}
