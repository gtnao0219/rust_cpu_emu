use crate::instruction;

// const EAX: usize = 0;
// const ECX: usize = 1;
// const EDX: usize = 2;
// const EBX: usize = 3;
const ESP: usize = 4;
// const EBP: usize = 5;
// const ESI: usize = 6;
// const EDI: usize = 7;
const REGISTERS_COUNT: usize = 8;

#[derive(Debug)]
pub struct Emulator {
    pub registers: [u32; REGISTERS_COUNT],
    pub elags: u32,
    pub memory: Vec<u8>,
    pub eip: u32,
}

impl Emulator {
    pub fn new(program: &mut Vec<u8>, eip: u32, esp: u32) -> Emulator {
        let mut memory = vec![0; eip as usize];
        memory.append(program);
        let mut emu = Emulator{
            registers: [0; REGISTERS_COUNT],
            elags: 0,
            memory: memory,
            eip: eip,
        };
        emu.registers[ESP] = esp;
        emu
    }
    pub fn start(&mut self) -> () {
        loop {
            instruction::next(self);
            if self.eip == 0x00 || self.eip as usize >= self.memory.len() {
                break;
            }
        }
        println!("{:?}", self.registers);
    }
}
