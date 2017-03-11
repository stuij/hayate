// bios
pub const BIOS_INSTR_START: u32 = 0x00000000;
pub const BIOS_INSTR_LEN:   u32 = 0x00080000;
pub const BIOS_INSTR_END:   u32 = BIOS_INSTR_START + BIOS_INSTR_LEN;

// game flashrom
pub const GAME_INSTR_START: u32 = 0x06000000;
pub const GAME_INSTR_LEN:   u32 = 0x01000000;
pub const GAME_INSTR_END:   u32 = GAME_INSTR_START + GAME_INSTR_LEN;

// main ram
pub const MAIN_RAM_START:   u32 = 0x02000000;
pub const MAIN_RAM_LEN:     u32 = 0x00080000;
pub const MAIN_RAM_END:     u32 = MAIN_RAM_START + MAIN_RAM_LEN;
