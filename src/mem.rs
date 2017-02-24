extern crate zip;


use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path;

use byteorder::{BigEndian, ByteOrder};
use game_info::*;
use mem_defs;


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
    let addr_xor = addr ^ key1;
    let mut val: u16 = addr_xor as u16 ^ 0xffff;
    val = rotxor(val, key2 as u16);
    val ^= (addr_xor >> 16) as u16 ^ 0xffff;
	  val = rotxor(val, (key2 >> 16) as u16);
	  val ^= addr_xor as u16 ^ key2 as u16;
	  let ret = val as u32 | ((val as u32) << 16);
    ret
}


fn decrypt_instructions(buff: &mut [u8], addr_start: u32, key: &GameKey) {
    for i in (0..buff.len()).filter(|&x| x % 4 == 0) {
        let word = BigEndian::read_u32(&buff[i..i+4]);
        let unmasked = word ^ cps3_mask(addr_start+i as u32, key.a, key.b);
        BigEndian::write_u32(&mut buff[i..i+4], unmasked);
    }
}


// zip file extraction
fn check_data_crc(data: &DataSlice, zip: &mut zip::ZipArchive<fs::File>) {
    let file = zip.by_name(data.name.as_str()).unwrap();
    assert_eq!(file.crc32(), data.crc);
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


// mangling bits
fn mangle_bios_code(name: &str,
                    key: &GameKey,
                    zip: &mut zip::ZipArchive<fs::File>)
                    -> Vec<u8> {
    let mut bios = read_zip(name, zip);
    assert!(bios.len() == mem_defs::BIOS_INSTR_LEN as usize);
    decrypt_instructions(&mut bios, mem_defs::BIOS_INSTR_START as u32, key);
    bios
}


fn interlace_game_code(instr: &mut Vec<u8>,
                  zip: &mut zip::ZipArchive<fs::File>,
                  a_info: &DataSlice,
                  b_info: &DataSlice,
                  c_info: &DataSlice,
                  d_info: &DataSlice) {
    let a = read_zip(a_info.name.as_str(), zip);
    let b = read_zip(b_info.name.as_str(), zip);
    let c = read_zip(c_info.name.as_str(), zip);
    let d = read_zip(d_info.name.as_str(), zip);

    for (i, j, k, l) in izip!(a, b, c, d) {
        instr.push(i);
        instr.push(j);
        instr.push(k);
        instr.push(l);
    }
}


fn mangle_game_code(data: &Vec<DataSlice> ,
                    key: &GameKey,
                    zip: &mut zip::ZipArchive<fs::File>)
                    -> Vec<u8> {
    let code_size: u32 = data.iter().map(|d| d.size).sum();
    println!("game code size: 0x{:x}", code_size);

    // interlace the 8 game data flashrom bytes, a byte
    // at the time; the one after the other
    let mut instr = Vec::with_capacity(mem_defs::GAME_INSTR_LEN as usize);

    interlace_game_code(&mut instr, zip, &data[0], &data[1], &data[2], &data[3]);
    interlace_game_code(&mut instr, zip, &data[4], &data[5], &data[6], &data[7]);

    // and decrypt
    decrypt_instructions(&mut instr, mem_defs::GAME_INSTR_START, key);

    instr
}


fn interlace_gfx(instr: &mut Vec<u8>,
                  zip: &mut zip::ZipArchive<fs::File>,
                  a_info: &DataSlice,
                  b_info: &DataSlice) {
    let a = read_zip(a_info.name.as_str(), zip);
    let b = read_zip(b_info.name.as_str(), zip);

    for (i, j) in izip!(a, b) {
        instr.push(i);
        instr.push(j);
    }
}


fn mangle_gfx_data(data: &Vec<DataSlice>, zip: &mut zip::ZipArchive<fs::File>)
                   -> Vec<u8> {
    let gfx_size: u32 = data.iter().map(|d| d.size).sum();
    println!("game gfx size: 0x{:x}", gfx_size);

    let mut gfx = Vec::with_capacity(gfx_size as usize);

    for i in (0..data.len()).filter(|&x| x % 2 == 0) {
        interlace_gfx(&mut gfx, zip, &data[i], &data[i+1]);
    }

    gfx
}


fn write_bin(bin: &[u8], path_string: &str) {
    let path = path::Path::new(path_string);
    let display = path.display();

    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(bin) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}


fn read_bin(path_string: &str) -> Vec<u8> {
    let path = path::Path::new(path_string);
    let display = path.display();
    let mut bin = Vec::new();
    
    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}",
                           display,
                           why.description()),
        Ok(file) => file
    };

    match file.read_to_end(&mut bin) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => bin,
    }
}


// interface
pub fn dump_binaries(info: GameInfo, rom: GameRom) {
    fs::create_dir_all(&info.id).unwrap();
    write_bin(&rom.bios, &[&info.id, "bios.bin"].join("/"));
    write_bin(&rom.instr, &[&info.id, "instr.bin"].join("/"));
    write_bin(&rom.gfx, &[&info.id, "gfx.bin"].join("/"));
}


pub fn mem_from_bins(info: &GameInfo) -> GameRom {
    // do stuff from here
    GameRom{
        bios:  read_bin(&[&info.path, "bios.bin"].join("/")).into_boxed_slice(),
        instr: read_bin(&[&info.path, "instr.bin"].join("/")).into_boxed_slice(),
        gfx:   read_bin(&[&info.path, "gfx.bin"].join("/")).into_boxed_slice(),
    }
}


pub fn mem_from_zip(info: &GameInfo) -> GameRom {

    // open game zip file
    let file = fs::File::open(&info.path).expect("Couldn't open game file.");
    let mut zip = zip::ZipArchive::new(file).unwrap();

    // sanity-check the rom data, and convert it to the format
    // we will use in the emulator
    check_game_crcs(&info, &mut zip);
    let game_code = mangle_game_code(&info.instr, &info.key, &mut zip);
    let bios_code = mangle_bios_code(info.bios.name.as_str(),
                                         &info.key,
                                     &mut zip);
    let gfx_data = mangle_gfx_data(&info.gfx, &mut zip);

    GameRom { bios: bios_code.into_boxed_slice(),
              instr: game_code.into_boxed_slice(),
              gfx: gfx_data.into_boxed_slice() }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decrypt_word() {
        // the first word in our test binary
        let word = 0x491141b1;
        let mask = cps3_mask(mem_defs::GAME_INSTR_START,
                             0xa55432b4,
                             0x0c129981);
        let unmasked = word ^ mask;
        assert_eq!(0x06000ea0, unmasked);
    }
}
