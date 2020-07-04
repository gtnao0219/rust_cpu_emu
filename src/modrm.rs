use crate::emulator::*;

#[derive(Debug)]
pub struct ModRM {
    pub _mod: u8,
    pub opecode: u8,
    pub reg_index: u8,
    pub rm: u8,
    pub sib: u8,
    pub disp8: i8,
    pub disp32: u32,
}

pub fn parse_modrm(emu: &mut Emulator) -> ModRM {
    let mut modrm = ModRM{
        _mod: 0,
        opecode: 0,
        reg_index: 0,
        rm: 0,
        sib: 0,
        disp8: 0,
        disp32: 0,
    };
    let code = emu.get_code8(0);
    modrm._mod = (code & 0xC0) >> 6;
    modrm.opecode = (code & 0x38) >> 3;
    modrm.reg_index = (code & 0x38) >> 3;
    modrm.rm = code & 0x07;

    emu.eip += 1;

    if modrm._mod != 3 && modrm.rm == 4 {
        modrm.sib = emu.get_code8(0);
        emu.eip += 1;
    }

    if (modrm._mod == 0 && modrm.rm == 5) || modrm._mod == 2 {
        modrm.disp32 = emu.get_code32(0);
        emu.eip += 4;
    } else if modrm._mod == 1 {
        modrm.disp8 = emu.get_sign_code8(0);
        emu.eip += 1;
    }

    modrm
}

impl ModRM {
    pub fn calc_memory_address(&self, emu: &Emulator) -> u32 {
        if self._mod == 0 {
            if self.rm == 4 {
                panic!("not implemented Modrm mod = 0, rm = 4");
            } else if self.rm == 5 {
                self.disp32
            } else {
                emu.get_register32(self.rm as usize)
            }
        } else if self._mod == 1 {
            if self.rm == 4 {
                panic!("not implemented Modrm mod = 1, rm = 4");
            } else {
                ((emu.get_register32(self.rm as usize) as i32) + (self.disp8 as i32)) as u32
            }
        } else if self._mod == 2 {
            if self.rm == 4 {
                panic!("not implemented Modrm mod = 2, rm = 4");
            } else {
                emu.get_register32(self.rm as usize) + self.disp32
            }
        } else {
            panic!("not implemented Modrm mod = 3");
        }
    }
    pub fn set_rm32(&self, emu: &mut Emulator, value: u32) -> () {
        if self._mod == 3 {
            emu.set_register_32(self.rm as usize, value);
        } else {
            let address = self.calc_memory_address(emu);
            emu.set_memory32(address as usize, value);
        }
    }
    pub fn set_r32(&self, emu: &mut Emulator, value: u32) -> () {
        emu.set_register_32(self.reg_index as usize, value);
    }
    pub fn get_rm32(&self, emu: &mut Emulator) -> u32 {
        if self._mod == 3 {
            emu.get_register32(self.rm as usize)
        } else {
            let address = self.calc_memory_address(emu);
            emu.get_memory32(address as usize)
        }
    }
    pub fn get_r32(&self, emu: &mut Emulator) -> u32 {
        emu.get_register32(self.reg_index as usize)
    }
}