use thalgar;

use game_info;
use mem_defs::*;


pub struct Cps3Mem {
    pub rom: game_info::GameRom, // flashroms
    pub main_ram: Box<[u8]>
}

impl Cps3Mem {
    pub fn new (rom: game_info::GameRom) -> Cps3Mem {
        Cps3Mem {
            rom: rom,
            main_ram: { let ram = vec![0; MAIN_RAM_LEN as usize];
                        ram.into_boxed_slice() }
        }
    }

    fn read_mem<T: thalgar::MemAccess>(&self, addr: u32) -> T {
        match addr {
            MAIN_RAM_START ... MAIN_RAM_END => {
                let offset = (addr ^ MAIN_RAM_START) as usize;
                T::read_mem(&self.main_ram, offset)
            },
            GAME_INSTR_START ... GAME_INSTR_END => {
                let offset = (addr ^ GAME_INSTR_START) as usize;
                T::read_mem(&self.rom.instr, offset)
            },
            _ => panic!("read address not mapped: {:#010x}", addr)
        }
    }

    fn write_mem<T: thalgar::MemAccess>(&mut self, addr: u32, val: T) {
        match addr {
            MAIN_RAM_START ... MAIN_RAM_END => {
                let offset = (addr ^ MAIN_RAM_START) as usize;
                T::write_mem(&mut self.main_ram, offset, val);
            },
            _ => panic!("write address not mapped: {:#010x}", addr)
        }
    }
}


impl thalgar::Bus for Cps3Mem {
    // this looks like overengineering, but this will actually
    // save us some trouble in the end, as we can't just map to mem
    // like this all the time
    fn read_byte(&self, addr: u32) -> u8 {
        self.read_mem::<u8>(addr)
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        self.write_mem::<u8>(addr, val)
    }

    fn read_word(&self, addr: u32) -> u16 {
        self.read_mem::<u16>(addr)
    }

    fn write_word(&mut self, addr: u32, val: u16) {
        match addr {
            0x040c0082 => (), // unknown video (?) register
            _ => self.write_mem::<u16>(addr, val)
        }

    }

    fn read_long(&self, addr: u32) -> u32 {
        self.read_mem::<u32>(addr)
    }

    fn write_long(&mut self, addr: u32, val: u32) {
        self.write_mem::<u32>(addr, val)
    }
}
