extern crate zip;

use std::fs;
use std::io::Read;

use common::{read_u16, read_u32, write_u16, write_u32};
use game_info::*;

// game instruction decryption
fn rotate_left(val: u16, n: i32) -> u16 {
    let aux: i32 = (val >> (16 - n)) as i32;
    (((val << n) as i32 | aux) & 0xffff) as u16
}

fn rotxor(val: u16, x: u16) -> u16 {
    let res = val.wrapping_add(rotate_left(val, 2));
    rotate_left(res, 4) ^ (res & (val ^ x))
}

fn cps3_mask(addr: u32, key1: u32, key2: u32) -> u32 {
    let mut addr_xor = addr ^ key1;
    let mut val: u16 = addr_xor as u16 ^ 0xffff;
    let mut val = rotxor(val, key2 as u16);
    val ^= (addr_xor >> 16) as u16 ^ 0xffff;
	  val = rotxor(val, (key2 >> 16) as u16);
	  val ^= addr_xor as u16 ^ key2 as u16;
	  let ret = val as u32 | ((val as u32) << 16);
    ret 
}

//fn decrypt_instructions(instructions: &mut vec, addr_start: u32) {
    
//}


// zip file extraction
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

    // The SH2 CPU is big-endian. If the CPU that does the emulation is
    // little-endian, we need to make a decision on if we want to convert
    // instructions in memory, or when we read the data.
    // Are we going for purity or for speed? We're going for speed,
    // so we'll convert now.

    //if cfg!(target_endian = "big") {
    //    for 
    //}
    
    // As we also need to decrypt the code, this is as good a place as any.
    instr

}

// entrypoint
pub fn init_mem(info: &GameInfo) {
    println!("rotxor: {}", rotxor(0xab04, 0x98fe));
    println!("cps3_mask: {}", cps3_mask(0, 0xb5fe053e, 0xfc03925a));

    let mut bla = vec!(0, 0, 0, 255, 0);
    println!("get u32: {:x}", read_u32(&bla[0..4]));

    write_u16(&mut bla[0..2], 0xFF);
    println!("write u16: {:?}", bla);
    write_u32(&mut bla[0..4], 0xEEEEEEEE);
    println!("write u16: {:?}", bla);

    return;

    
    // open game zip file
    let file = fs::File::open(&info.path).expect("Couldn't open game file.");
    let mut zip = zip::ZipArchive::new(file).unwrap();

    // sanity-check the rom data, and convert it to the format
    // we will use in the emulator
    check_game_crcs(&info, &mut zip);
    let bios_code = mangle_bios_code(info.bios.name.as_str(), &mut zip);
    let game_code = mangle_game_code(&info.instr, &mut zip);
}
