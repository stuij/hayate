use std::process;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::str;

use thalgar;
use thalgar::Bus;
use bus;

enum Cmd {
    Break { bkpt: u32 },
    ClearBkpt { bkpt: u32 },
    Disassemble { start: u32, end: u32 },
    Empty,
    Err { msg: &'static str },
    Info,
    List,
    Quit,
    Run,
    Step,
    Trace,
    Untrace,
    Unknown,
    View { start: u32, end: u32 },
    ViewStack,
}


pub struct Debugger {
    disasm: thalgar::Disassemble,
    bkpts: HashSet<u32>
}


impl Debugger {
    pub fn new() -> Debugger {
        Debugger { disasm: thalgar::Disassemble::new(),
                   bkpts: HashSet::new() }
    }

    fn get_input(&self) -> String {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        print!(">> ");
        io::stdout().flush().unwrap();

        // TODO: don't just loose the error info!
        handle.read_line(&mut buffer).unwrap_or(0);
        buffer
    }


    fn parse_break(&self, mut iter: str::SplitWhitespace) -> Cmd {
        match iter.next() {
            Some(x) => {
                match u32::from_str_radix(x, 16) {
                    Ok(x) => Cmd::Break { bkpt: x },
                    Err(_) => Cmd::Err { msg: "couldn't parse address"}
                }
            },
            None => Cmd::Err { msg: "no bkpt address"}
        }
    }


    fn parse_break_clear(&self, mut iter: str::SplitWhitespace) -> Cmd {
        match iter.next() {
            Some(x) => {
                match u32::from_str_radix(x, 16) {
                    Ok(x) => Cmd::ClearBkpt { bkpt: x },
                    Err(_) => Cmd::Err { msg: "couldn't parse address"}
                }
            },
            None => Cmd::Err { msg: "no bkpt address"}
        }
    }


    fn parse_disassemble(&self, mut iter: str::SplitWhitespace) -> Cmd {
        let first = match iter.next() {
            Some(x) => {
                match u32::from_str_radix(x, 16) {
                    Ok(x) => x,
                    Err(_) => return Cmd::Err {
                        msg: "couldn't parse first address"
                    },
                }
            },
            None => return Cmd::Err { msg: "no addresses given"}
        };

        let second = match iter.next() {
            Some(x) => {
                match u32::from_str_radix(x, 16) {
                    Ok(x) => x,
                    Err(_) => return Cmd::Err {
                        msg: "couldn't parse second address"
                    },
                }
            },
            None => return Cmd::Err { msg: "no second address given"}
        };

        Cmd::Disassemble { start: first, end: second }
    }


    fn parse_view(&self, mut iter: str::SplitWhitespace) -> Cmd {
        let what = match iter.next() {
            Some(x) => x,
            None => return Cmd::Err { msg: "no addresses given"}
        };

        if what == "stack" {
            return Cmd::ViewStack
        }

        let first = match u32::from_str_radix(what, 16) {
            Ok(x) => x,
            Err(_) => return Cmd::Err {
                msg: "couldn't parse first address"
            },
        };

        let second = match iter.next() {
            Some(x) => {
                match u32::from_str_radix(x, 16) {
                    Ok(x) => x,
                    Err(_) => return Cmd::Err {
                        msg: "couldn't parse second address"
                    },
                }
            },
            None => return Cmd::Err { msg: "no second address given"}
        };

        Cmd::View { start: first, end: second }
    }


    fn get_cmd(&self) -> Cmd {
        let input = self.get_input();
        let mut iter = input.split_whitespace();

        match iter.next() {
            Some("b") => self.parse_break(iter),
            Some("clear") => self.parse_break_clear(iter),
            Some("d") => self.parse_disassemble(iter),
            Some("i") => Cmd::Info,
            Some("l") => Cmd::List,
            Some("q") => Cmd::Quit,
            Some("r") => Cmd::Run,
            Some("s") => Cmd::Step,
            Some("t") => Cmd::Trace,
            Some("u") => Cmd::Untrace,
            Some("v") => self.parse_view(iter),
            Some(_) => Cmd::Unknown,
            None => Cmd::Empty,
        }
    }

    fn print_mem(&self, bus: &bus::Cps3Bus, start: u32, end: u32) {
        for i in (start..end).filter(|x| x % 4 == 0) {
            let val = bus.read_long(i);
            println!("{:#010x}: {:#010x}", i, val);
        }
    }

    fn view_stack(&self, bus: &bus::Cps3Bus, start: u32, end: u32) {
        self.print_mem(bus, end, start);
    }

    fn insert_bkpt(&mut self, bkpt: u32) {
        self.bkpts.insert(bkpt);
    }

    fn clear_bkpt(&mut self, bkpt: u32) {
        self.bkpts.remove(&bkpt);
    }

    pub fn debug(&mut self,
             cpu: &mut thalgar::Sh2,
             bus: &mut bus::Cps3Bus,
             mut run: bool) {

        let mut step = false;
        let mut trace = false;
        let stack_start = cpu.get_regs().gpr[15];

        // repl
        loop {
            // if run then just blast
            // let's keep this simplistic for now
            if run || step {
                if trace {
                    self.disasm.disasemble(bus, cpu.get_pc());
                }
                cpu.step(bus);
                if self.bkpts.contains(&cpu.get_pc()) {
                    run = false;
                };
                step = false;
            } else {
                let regs = cpu.get_regs();
                let pc = regs.pc;
                self.disasm.disasemble(bus, pc);
                let cmd = self.get_cmd();

                match cmd {
                    Cmd::Break { bkpt} => self.insert_bkpt(bkpt),
                    Cmd::ClearBkpt { bkpt } => self.clear_bkpt(bkpt),
                    Cmd::Disassemble { start, end } =>
                        self.disasm.disassemble_range(bus, start, end, pc),
                    Cmd::Empty => (),
                    Cmd::Info => println!("{}", cpu),
                    Cmd::List =>
                        self.disasm.disassemble_range(bus, pc-10, pc+20, pc),
                    Cmd::Quit => { println!("Ta.."); process::exit(0) },
                    Cmd::Run      => run = true,
                    Cmd::Step     => step = true,
                    Cmd::Trace    => trace = true,
                    Cmd::Untrace  => trace = false,
                    Cmd::Unknown  => println!("cmd not known"),
                    Cmd::ViewStack =>
                        self.view_stack(bus, stack_start, regs.gpr[15]),
                    Cmd::View {start, end} =>
                        self.disasm.disassemble_range(bus, start, end, pc),
                    Cmd::Err { msg } => println!("error: {}", msg)
                }
                println!();
            }
        }
    }
}
