use thalgar;

use common::MemAccess;
use debugger;
use game_info;
use mem;
use mem_defs::*;


fn reset(cpu: &mut thalgar::Sh2, bus: &thalgar::Sh7604Mem<mem::Cps3Mem>) {
    let pc = u32::read_mem(&bus.user.rom.instr, 0);
    let sp = u32::read_mem(&bus.user.rom.instr, 4);
    cpu.reset(pc, sp);
    cpu.set_vbr(GAME_INSTR_START);
}

pub fn launch(rom: game_info::GameRom) {
    let mut cpu = thalgar::Sh2::new();
    let cps3_mem = mem::Cps3Mem::new(rom);
    let mut sh7604_mem = thalgar::Sh7604Mem::new(cps3_mem);

    reset(&mut cpu, &sh7604_mem);

    let mut debugger = debugger::Debugger::new();
    debugger.debug(&mut cpu, &mut sh7604_mem, false);
}
