use crate::emulator::*;
use crate::modrm::*;

pub fn next(emu: &mut Emulator) -> () {
    let code = emu.get_code8(0);
    match code {
        0x01 => add_rm32_r32(emu),
        0x83 => code_83(emu),
        0x89 => mov_rm32_r32(emu),
        0x8B => mov_r32_rm32(emu),
        0xB8 => mov_r32_imm32(emu),
        0xB9 => mov_r32_imm32(emu),
        0xBA => mov_r32_imm32(emu),
        0xBB => mov_r32_imm32(emu),
        0xBC => mov_r32_imm32(emu),
        0xBD => mov_r32_imm32(emu),
        0xBE => mov_r32_imm32(emu),
        0xBF => mov_r32_imm32(emu),
        0xC7 => mov_rm32_imm32(emu),
        0xE9 => near_jump(emu),
        0xEB => short_jump(emu),
        0xFF => code_ff(emu),
        _ => {
        },
    }
}

// instructions
fn mov_r32_imm32(emu: &mut Emulator) -> () {
    let reg = emu.get_code8(0) - 0xB8;
    let value = emu.get_code32(1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}
fn mov_rm32_imm32(emu: &mut Emulator) -> () {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let value = emu.get_code32(0);
    emu.eip += 4;
    modrm.set_rm32(emu, value);
}
fn mov_rm32_r32(emu: &mut Emulator) -> () {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let r32 = modrm.get_r32(emu);
    modrm.set_rm32(emu, r32);
}
fn mov_r32_rm32(emu: &mut Emulator) -> () {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let rm32 = modrm.get_rm32(emu);
    modrm.set_r32(emu, rm32);
}
fn short_jump(emu: &mut Emulator) -> () {
    let diff = emu.get_sign_code8(1);
    emu.eip = ((emu.eip as i32) + (diff as i32) + 2) as u32;
}
fn near_jump(emu: &mut Emulator) -> () {
    let diff = emu.get_sign_code32(1);
    emu.eip = ((emu.eip as i32) + (diff as i32) + 5) as u32;
}
fn add_rm32_r32(emu: &mut Emulator) -> () {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let r32 = modrm.get_r32(emu);
    let rm32 = modrm.get_rm32(emu);
    modrm.set_rm32(emu, r32 + rm32);
}
fn code_83(emu: &mut Emulator) -> () {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    if modrm.opecode == 5 {
        sub_rm32_imm8(emu, &modrm);
    } else {
        panic!("not implemented: 83 {:?}", modrm.opecode);
    }
}
fn sub_rm32_imm8(emu: &mut Emulator, modrm: & ModRM) -> () {
    let rm32 = modrm.get_rm32(emu);
    let imm8 = emu.get_sign_code8(0);
    emu.eip += 1;
    modrm.set_rm32(emu, ((rm32 as i32) - (imm8 as i32)) as u32);
}
fn code_ff(emu: &mut Emulator) -> () {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    if modrm.opecode == 0 {
        inc_rm32(emu, &modrm);
    } else {
        panic!("not implemented: FF {:?}", modrm.opecode);
    }
}
fn inc_rm32(emu: &mut Emulator, modrm: & ModRM) -> () {
    let value = modrm.get_rm32(emu);
    modrm.set_rm32(emu, value + 1);
}