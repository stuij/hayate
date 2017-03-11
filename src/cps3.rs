use thalgar;

use bus;
use common::MemAccess;
use debugger;
use game_info;
use mem_defs::*;


fn reset(cpu: &mut thalgar::Sh2, bus: &bus::Cps3Bus) {
    let pc = u32::read_mem(&bus.rom.instr, 0);
    let sp = u32::read_mem(&bus.rom.instr, 4);
    cpu.reset(pc, sp);
    cpu.set_vbr(GAME_INSTR_START);
}

pub fn launch(rom: game_info::GameRom) {
    let mut cpu = thalgar::Sh2::new();
    let mut bus = bus::Cps3Bus::new(rom);

    reset(&mut cpu, &bus);

    let mut debugger = debugger::Debugger::new();
    debugger.debug(&mut cpu, &mut bus, false);
}
