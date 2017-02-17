#[macro_use] extern crate itertools;
extern crate zip;

use std::env;
use std::fs;
use std::io::Read;

struct DataSlice {
    name: String,
    size: u32,
    crc:  u32
}

struct GameKey {
    a: u32,
    b: u32
}

struct GameInfo {
    id:   String,
    path: String,
    name: String,
    key:  GameKey,
    bios: DataSlice,
    instr: Vec<DataSlice>,
    gfx:  Vec<DataSlice>
}

struct Rom {
    bios:       Box<u8>,
    game_instr: Box<u8>,
    game_gfx:   Box<u8>
}

fn check_data_crc(data: &DataSlice, zip: &mut zip::ZipArchive<fs::File>) {
    let bios_file = zip.by_name(data.name.as_str()).unwrap();
    assert_eq!(bios_file.crc32(), data.crc);
}


fn check_game_crcs(info: &GameInfo, zip: &mut zip::ZipArchive<fs::File>) {
    check_data_crc(&info.bios, zip);
    for data in &info.instr {
        check_data_crc(&data, zip);
    }
    for data in &info.gfx {
        check_data_crc(&data, zip);
    }
}

fn read_zip(name: &str, zip: &mut zip::ZipArchive<fs::File>)
                    -> Vec<u8> {
    let mut data = Vec::new();
    let mut data_file = zip.by_name(name).unwrap();
    let size = data_file.read_to_end(&mut data).unwrap();
    println!("File size: 0x{:x}", size);
    data
}


fn mangle_bios_code(name: &str, zip: &mut zip::ZipArchive<fs::File>)
                    -> Vec<u8> {
    read_zip(name, zip)
}


fn interlace_game_code(instr: &mut Vec<u8>,
                  zip: &mut zip::ZipArchive<fs::File>,
                  a_in: &DataSlice,
                  b_in: &DataSlice,
                  c_in: &DataSlice,
                  d_in: &DataSlice) {
    let a = read_zip(a_in.name.as_str(), zip);
    let b = read_zip(b_in.name.as_str(), zip);
    let c = read_zip(c_in.name.as_str(), zip);
    let d = read_zip(d_in.name.as_str(), zip);

    for (i, j, k, l) in izip!(a, b, c, d) {
        instr.push(i);
        instr.push(j);
        instr.push(k);
        instr.push(l);
    }
}


fn mangle_game_code(data: &Vec<DataSlice> , zip: &mut zip::ZipArchive<fs::File>)
                    -> Vec<u8> {
    let code_size: u32 = data.iter().map(|d| d.size).sum();
    println!("game data size: 0x{:x}", code_size);

    // interlace the 8 game data flashrom bytes, a byte
    // at the time; the one after the other
    let mut instr = Vec::with_capacity(0x1000000);

    interlace_game_code(&mut instr, zip, &data[0], &data[1], &data[2], &data[3]);
    interlace_game_code(&mut instr, zip, &data[4], &data[5], &data[6], &data[7]);

    instr
}


fn init_mem(info: &GameInfo) {
    // open game zip file
    let file = fs::File::open(&info.path).expect("Couldn't open game file.");
    let mut zip = zip::ZipArchive::new(file).unwrap();

    // sanity-check the rom data, and convert it to the format
    // we will use in the emulator
    check_game_crcs(&info, &mut zip);
    let bios_code = mangle_bios_code(info.bios.name.as_str(), &mut zip);
    let game_code = mangle_game_code(&info.instr, &mut zip);
}


fn init(info: &GameInfo) {
    init_mem(info);
}

fn help() {
    println!("usage:
  hayate <CPS-3 game zipfile>");
}

fn main() {    
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let sfiiin = get_game_info(args[1].to_string());
            init(&sfiiin)
        },
        _ => {
            help();
        }
    };
}

fn get_game_info(path: String) -> GameInfo {
    GameInfo {
        id: String::from("sfiiin"),
        path: path,
        name: String::from("Street Fighter III: New Generation (Asia 970204, NO CD, bios set 1)"),
        key: GameKey { a: 0xb5fe053e, b: 0xfc03925a },
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
