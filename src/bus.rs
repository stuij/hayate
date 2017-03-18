use thalgar;

use common;
use common::MemAccess;
use game_info;
use mem_defs::*;


pub struct Cps3Bus {
    pub rom: game_info::GameRom, // flashroms
    pub main_ram: Box<[u8]>
}

impl Cps3Bus {
    pub fn new (rom: game_info::GameRom) -> Cps3Bus {
        Cps3Bus {
            rom: rom,
            main_ram: { let ram = vec![0; MAIN_RAM_LEN as usize];
                        ram.into_boxed_slice() }
        }
    }

    fn read_mem<T: common::MemAccess>(&self, addr: u32) -> T {
        match addr {
            MAIN_RAM_START ... MAIN_RAM_END => {
                let offset = (addr ^ MAIN_RAM_START) as usize;
                T::read_mem(&self.main_ram, offset)
            },
            GAME_INSTR_START ... GAME_INSTR_END => {
                let offset = (addr ^ GAME_INSTR_START) as usize;
                T::read_mem(&self.rom.instr, offset)
            },
            _ => panic!("address not mapped: {:#010x}", addr)
        }
    }

    fn write_mem<T: common::MemAccess>(&mut self, addr: u32, val: T) {
        match addr {
            MAIN_RAM_START ... MAIN_RAM_END => {
                let offset = (addr ^ MAIN_RAM_START) as usize;
                T::write_mem(&mut self.main_ram, offset, val);
            },
            _ => panic!("address not mapped: {:#010x}", addr)
        }
    }
}


impl thalgar::Bus for Cps3Bus {
    fn read_word(&self, addr: u32) -> u16 {
        // this looks like overengineering, but this will actually
        // save us some trouble in the end, as we can't just map to mem
        // like this all the time
        self.read_mem::<u16>(addr)
    }

    fn read_long(&self, addr: u32) -> u32 {
        // see read_word
        self.read_mem::<u32>(addr)
    }

    fn write_long(&mut self, addr: u32, val: u32) {
        // see read_word
        self.write_mem::<u32>(addr, val)
    }
}
