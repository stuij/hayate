
pub struct DataSlice {
    pub name: String,
    pub size: u32,
    pub crc:  u32
}

pub struct GameKey {
    pub a: u32,
    pub b: u32
}

pub struct GameInfo {
    pub id:   String,
    pub path: String,
    pub name: String,
    pub key:  GameKey,
    pub bios: DataSlice,
    pub instr: Vec<DataSlice>,
    pub gfx:  Vec<DataSlice>
}

pub struct GameRom {
    pub bios:  Box<[u8]>,
    pub instr: Box<[u8]>,
    pub gfx:   Box<[u8]>
}

pub fn get_game_info(path: String) -> GameInfo {
    GameInfo {
        id: String::from("sfiiin"),
        path: path,
        name: String::from("Street Fighter III 3rd Strike: Fight for the Future (Japan 990608, NO CD)"),
        key: GameKey { a: 0xa55432b4, b: 0x0c129981 },
        bios: DataSlice { name: String::from("sfiii3_japan_nocd.29f400.u2"),
                          size: 0x080000,
                          crc:  0x1edc6366 },
        instr: vec![
            DataSlice { name: String::from("sfiii3n-simm1.0"),
	                      size: 0x200000,
                        crc:  0x11dfd3cd },
            DataSlice { name: String::from("sfiii3n-simm1.1"),
	                      size: 0x200000,
                        crc:  0xc50585e6 },
            DataSlice { name: String::from("sfiii3n-simm1.2"),
	                      size: 0x200000,
                        crc:  0x8e011d9b },
            DataSlice { name: String::from("sfiii3n-simm1.3"),
	                      size: 0x200000,
                        crc:  0xdca8d92f },
            DataSlice { name: String::from("sfiii3n-simm2.0"),
	                      size: 0x200000,
                        crc:  0x06eb969e },
            DataSlice { name: String::from("sfiii3n-simm2.1"),
	                      size: 0x200000,
                        crc:  0xe7039f82 },
            DataSlice { name: String::from("sfiii3n-simm2.2"),
	                      size: 0x200000,
                        crc:  0x645c96f7 },
            DataSlice { name: String::from("sfiii3n-simm2.3"),
	                      size: 0x200000,
                        crc:  0x610efab1 },    
        ],

        gfx: vec![
            DataSlice { name: String::from("sfiii3n-simm3.0"),
	                      size: 0x200000,
                        crc:  0x7baa1f79 },
            DataSlice { name: String::from("sfiii3n-simm3.1"),
	                      size: 0x200000,
                        crc:  0x234bf8fe },
            DataSlice { name: String::from("sfiii3n-simm3.2"),
	                      size: 0x200000,
                        crc:  0xd9ebc308 },
            DataSlice { name: String::from("sfiii3n-simm3.3"),
	                      size: 0x200000,
                        crc:  0x293cba77 },
            DataSlice { name: String::from("sfiii3n-simm3.4"),
	                      size: 0x200000,
                        crc:  0x6055e747 },
            DataSlice { name: String::from("sfiii3n-simm3.5"),
	                      size: 0x200000,
                        crc:  0x499aa6fc },
            DataSlice { name: String::from("sfiii3n-simm3.6"),
	                      size: 0x200000,
                        crc:  0x6c13879e },
            DataSlice { name: String::from("sfiii3n-simm3.7"),
	                      size: 0x200000,
                        crc:  0xcf4f8ede },
            DataSlice { name: String::from("sfiii3n-simm4.0"),
	                      size: 0x200000,
                        crc:  0x091fd5ba },
            DataSlice { name: String::from("sfiii3n-simm4.1"),
	                      size: 0x200000,
                        crc:  0x0bca8917 },
            DataSlice { name: String::from("sfiii3n-simm4.2"),
	                      size: 0x200000,
                        crc:  0xa0fd578b },
            DataSlice { name: String::from("sfiii3n-simm4.3"),
	                      size: 0x200000,
                        crc:  0x4bf8c699 },
            DataSlice { name: String::from("sfiii3n-simm4.4"),
	                      size: 0x200000,
                        crc:  0x137b8785 },
            DataSlice { name: String::from("sfiii3n-simm4.5"),
	                      size: 0x200000,
                        crc:  0x4fb70671 },
            DataSlice { name: String::from("sfiii3n-simm4.6"),
	                      size: 0x200000,
                        crc:  0x832374a4 },
            DataSlice { name: String::from("sfiii3n-simm4.7"),
	                      size: 0x200000,
                        crc:  0x1c88576d },
            DataSlice { name: String::from("sfiii3n-simm5.0"),
	                      size: 0x200000,
                        crc:  0xc67d9190 },
            DataSlice { name: String::from("sfiii3n-simm5.1"),
	                      size: 0x200000,
                        crc:  0x6cb79868 },
            DataSlice { name: String::from("sfiii3n-simm5.2"),
	                      size: 0x200000,
                        crc:  0xdf69930e },
            DataSlice { name: String::from("sfiii3n-simm5.3"),
	                      size: 0x200000,
                        crc:  0x333754e0 },
            DataSlice { name: String::from("sfiii3n-simm5.4"),
	                      size: 0x200000,
                        crc:  0x78f6d417 },
            DataSlice { name: String::from("sfiii3n-simm5.5"),
	                      size: 0x200000,
                        crc:  0x8ccad9b1 },
            DataSlice { name: String::from("sfiii3n-simm5.6"),
	                      size: 0x200000,
                        crc:  0x85de59e5 },
            DataSlice { name: String::from("sfiii3n-simm5.7"),
	                      size: 0x200000,
                        crc:  0xee7e29b3 },
            DataSlice { name: String::from("sfiii3n-simm6.0"),
	                      size: 0x200000,
                        crc:  0x8da69042 },
            DataSlice { name: String::from("sfiii3n-simm6.1"),
	                      size: 0x200000,
                        crc:  0x1c8c7ac4 },
            DataSlice { name: String::from("sfiii3n-simm6.2"),
	                      size: 0x200000,
                        crc:  0xa671341d },
            DataSlice { name: String::from("sfiii3n-simm6.3"),
	                      size: 0x200000,
                        crc:  0x1a990249 },
            DataSlice { name: String::from("sfiii3n-simm6.4"),
	                      size: 0x200000,
                        crc:  0x20cb39ac },
            DataSlice { name: String::from("sfiii3n-simm6.5"),
	                      size: 0x200000,
                        crc:  0x5f844b2f },
            DataSlice { name: String::from("sfiii3n-simm6.6"),
	                      size: 0x200000,
                        crc:  0x450e8d28 },
            DataSlice { name: String::from("sfiii3n-simm6.7"),
	                      size: 0x200000,
                        crc:  0xcc5f4187 },
        ]
    }
}
