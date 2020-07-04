use crate::emulator::*;

pub fn next(emu: &mut Emulator) -> () {
    let code = get_code8(&emu, 0);
    match code {
        0xB8 => mov_r32_imm32(emu),
        0xB9 => mov_r32_imm32(emu),
        0xBA => mov_r32_imm32(emu),
        0xBB => mov_r32_imm32(emu),
        0xBC => mov_r32_imm32(emu),
        0xBD => mov_r32_imm32(emu),
        0xBE => mov_r32_imm32(emu),
        0xBF => mov_r32_imm32(emu),
        0xE9 => near_jump(emu),
        0xEB => short_jump(emu),
        _ => {},
    }
}

// instructions
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

// helpers
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
