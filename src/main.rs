extern crate byteorder;
extern crate clap;
#[macro_use] extern crate itertools;
extern crate thalgar;
extern crate zip;

mod mem;
mod cps3;
mod debugger;
mod game_info;
mod mem_defs;
mod rom;


fn assert_rom(rom: &game_info::GameRom) {
    assert!(rom.bios[..4]  == [0x00, 0x00, 0x04, 0x00]);
    assert!(rom.instr[..4] == [0x06, 0x00, 0x0e, 0xa0]);
    assert!(rom.gfx[..4]   == [0x01, 0x04, 0xfd, 0xff]);

    assert!(rom.bios.len()  == 0x00080000);
    assert!(rom.instr.len() == 0x01000000);
    assert!(rom.gfx.len()   == 0x04000000);
}


fn make_arg_matcher<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("Hayate")
        .version("0.0.1")
        .author("Ties Stuij <ties@stuij.se>")
        .about("A rusty CPS-3 arcade board / Hitachi SH2 risc cpu emulator.")
        .arg(clap::Arg::with_name("input")
             .help("The game file or directory to read from or write to.")
             .required(true)
             .index(1))
        .arg(clap::Arg::with_name("dump-bins")
             .long("dump")
             .help("Dump the zip file given as input and dumps massaged binaries that can easily be read and used by Hayate."))
        .arg(clap::Arg::with_name("from-bins")
             .conflicts_with("dump-bins")
             .long("from-bins")
             .help("If set, the input argument is interpreted as a directory instead of a file. The directory should contain bin files made by the hayate `dump' command.")).get_matches()
}


fn init(info: game_info::GameInfo, rom: game_info::GameRom) {
    // just a smithering of test rom assertions
    assert_rom(&rom);
    println!("\nstarting: {}\nbios: {}\n", info.name, info.bios.name);
    cps3::launch(rom);
}


fn main() {
    let matches = make_arg_matcher();
    let from_bins_p = matches.is_present("from-bins");
    let dump_p      = matches.is_present("dump-bins");

    let sfiiin = game_info::get_game_info(matches
                                          .value_of("input")
                                          .unwrap()
                                          .to_string());

    let bins = if from_bins_p {
        rom::mem_from_bins(&sfiiin)
    } else {
        rom::mem_from_zip(&sfiiin)
    };

    if dump_p {
        rom::dump_binaries(sfiiin, bins);
    } else {
        init(sfiiin, bins);
    };
}
