use std::process;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use thalgar;

use bus;

enum Cmd {
    Quit,
    Run,
    State,
    Step,
    Unknown,
    Error,
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
            "quit"  => Cmd::Quit,
            "run"   => Cmd::Run,
            "state" => Cmd::State,
            "step"  => Cmd::Step,
            _       => Cmd::Unknown,
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
                    Cmd::Error => { println!("Error"); process::exit(1) },
                    Cmd::Quit => { println!("Ta.."); process::exit(0) },
                    Cmd::Step => step = true,
                    Cmd::State => println!("{}", cpu),
                    Cmd::Run => run = true,
                    Cmd::Unknown => println!("cmd not known")
                }
                println!();
            }
        }
    }
}
