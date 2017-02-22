extern crate byteorder;
#[macro_use] extern crate itertools;

use std::env;

mod common;
mod game_info;
mod mem;
mod mem_defs;

fn init(info: &game_info::GameInfo) {
    mem::init_mem(info);
}

fn help() {
    println!("usage:
  hayate <CPS-3 game zipfile>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let sfiiin = game_info::get_game_info(args[1].to_string());
            init(&sfiiin)
        },
        _ => {
            help();
        }
    };
}
