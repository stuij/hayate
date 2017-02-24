extern crate byteorder;
extern crate clap;
#[macro_use] extern crate itertools;

mod game_info;
mod mem;
mod mem_defs;

use clap::{Arg, App};
use game_info::*;


fn init(info: game_info::GameInfo, rom: GameRom) {
    println!("We're supposed to start here..");

    println!("bios start: {:?}", &rom.bios[..4]);
    println!("instr start: {:?}", &rom.instr[..4]);
    println!("gfx start: {:?}", &rom.gfx[..4]);
}


fn main() {

    let matches = App::new("Hayate")
        .version("0.0.1")
        .author("Ties Stuij <ties@stuij.se>")
        .about("A rusty CPS-3 arcade board / Hitachi SH2 risc cpu emulator.")
        .arg(Arg::with_name("input")
             .help("The game file or directory to read from or write to.")
             .required(true)
             .index(1))
        .arg(Arg::with_name("dump-bins")
             .long("dump")
             .help("Dump the zip file given as input and dumps massaged binaries that can easily be read and used by Hayate."))
        .arg(Arg::with_name("from-bins")
             .conflicts_with("dump-bins")
             .long("from-bins")
             .help("If set, the input argument is interpreted as a directory instead of a file. The directory should contain bin files made by the hayate `dump' command."))
        .get_matches();

    let from_bins_p = matches.is_present("from-bins");
    let dump_p      = matches.is_present("dump-bins");

    let sfiiin = game_info::get_game_info(matches.value_of("input").unwrap().to_string());

    let bins = if from_bins_p {
        mem::mem_from_bins(&sfiiin)
    } else {
        mem::mem_from_zip(&sfiiin)
    };

    if dump_p {
        mem::dump_binaries(sfiiin, bins);
    } else {
        init(sfiiin, bins);
    };
}
