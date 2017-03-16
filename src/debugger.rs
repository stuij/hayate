use std::process;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use thalgar;
use thalgar::Bus;
use bus;

enum Cmd {
    Error,
    Overview,
    Quit,
    Run,
    Step,
    Unknown,
    View { start: u32, end: u32 },
}


pub struct Debugger {
    disasm: thalgar::Disassemble,
    bpts: HashSet<u32>
}


impl Debugger {
    pub fn new() -> Debugger {
        Debugger { disasm: thalgar::Disassemble,
                   bpts: HashSet::new() }
    }

    fn get_input(&self) -> String {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        print!(">> ");
        io::stdout().flush().unwrap();

        // don't just loose the error info!
        handle.read_line(&mut buffer).unwrap_or(0);
        buffer
    }

    fn get_cmd(&self) -> Cmd {
        let mut input = self.get_input();
        let mut iter = input.split_whitespace();

        // maybe not unwrap!
        match iter.next().unwrap() {
            "o" => Cmd::Overview,
            "q" => Cmd::Quit,
            "r" => Cmd::Run,
            "s" => Cmd::Step,
            "v" => Cmd::View {
                start: u32::from_str_radix(iter.next().unwrap(), 16).unwrap(),
                end: u32::from_str_radix(iter.next().unwrap(), 16).unwrap()
            },
            _   => Cmd::Unknown,
        }
    }

    fn print_mem(&self, bus: &bus::Cps3Bus, start: u32, end: u32) {
        for i in (start..end).filter(|x| x % 4 == 0) {
            let val = bus.read_long(i);
            println!("{:#010x}: {:#010x}", i, val);
        }
    }

    pub fn debug(&mut self,
             cpu: &mut thalgar::Sh2,
             bus: &mut bus::Cps3Bus,
             mut run: bool) {

        let mut step = false;

        print!("\n-> ");
        self.disasm.disasemble(cpu, bus);

        // repl
        loop {
            // if run then just blast
            // let's keep this simplistic for now
            if run || step {
                cpu.step(bus);

                print!("-> ");
                self.disasm.disasemble(cpu, bus);
                step = false;
            } else {
                let cmd = self.get_cmd();

                match cmd {
                    Cmd::Error    => { println!("Error"); process::exit(1) },
                    Cmd::Overview => println!("{}", cpu),
                    Cmd::Quit     => { println!("Ta.."); process::exit(0) },
                    Cmd::Step     => step = true,
                    Cmd::Run      => run = true,
                    Cmd::Unknown  => println!("cmd not known"),
                    Cmd::View {start, end} => self.print_mem(bus, start, end),
                }
                println!();
            }
        }
    }
}
