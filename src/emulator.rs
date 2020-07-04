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
    // getter
    pub fn get_code8(&self, index: usize) -> u8 {
        self.memory[(self.eip as usize) + index]
    }
    pub fn get_sign_code8(&self, index: usize) -> i8 {
        self.get_code8(index) as i8
    }
    pub fn get_code32(&self, index: usize) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8)
        }
        ret
    }
    pub fn get_sign_code32(&self, index: usize) -> i32 {
        self.get_code32(index) as i32
    }
    pub fn get_register32(&self, index: usize) -> u32 {
        self.registers[index]
    }
    pub fn get_memory8(&self, address: usize) -> u8 {
        self.memory[address]
    }
    pub fn get_memory32(&self, address: usize) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_memory8(address + i) as u32) << (i * 8)
        }
        ret
    }
    // setter
    pub fn set_register_32(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
    }
    pub fn set_memory8(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }
    pub fn set_memory32(&mut self, address: usize, value: u32) {
        for i in 0..4 {
            self.set_memory8(address + i, (value >> (i * 8)) as u8);
        }
    }
}
