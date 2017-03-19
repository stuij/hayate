use std::process;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use thalgar;
use thalgar::Bus;
use bus;

enum Cmd {
    Break { bkpt: u32 },
    Disassemble { start: u32, end: u32 },
    Error,
    Overview,
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

    fn get_cmd(&self) -> Cmd {
        let mut input = self.get_input();
        let mut iter = input.split_whitespace();

        // TODO: maybe not unwrap!
        match iter.next().unwrap() {
            "b" => Cmd::Break {
                bkpt: u32::from_str_radix(iter.next().unwrap(), 16).unwrap()
            },
            "d" => Cmd::Disassemble {
                start: u32::from_str_radix(iter.next().unwrap(), 16).unwrap(),
                end:   u32::from_str_radix(iter.next().unwrap(), 16).unwrap(),
            },
            "o" => Cmd::Overview,
            "q" => Cmd::Quit,
            "r" => Cmd::Run,
            "s" => Cmd::Step,
            "t" => Cmd::Trace,
            "u" => Cmd::Untrace,
            "v" =>  {
                let first = iter.next().unwrap();
                if first == "stack" {
                    Cmd::ViewStack
                } else {
                    Cmd::View {
                        start: u32::from_str_radix(first, 16).unwrap(),
                        end: u32::from_str_radix(iter.next().unwrap(), 16).unwrap()
                    }
                }
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

    fn print_instr(&mut self, bus: &mut bus::Cps3Bus,
                   start: u32, end: u32) {
        self.disasm.print_range(bus, start, end);
    }

    fn view_stack(&self, bus: &bus::Cps3Bus, start: u32, end: u32) {
        self.print_mem(bus, end, start);
    }

    fn insert_bkpt(&mut self, bkpt: u32) {
        self.bkpts.insert(bkpt);
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
                    self.disasm.disasemble(cpu, bus);
                }
                cpu.step(bus);
                if self.bkpts.contains(&cpu.get_pc()) {
                    run = false;
                };
                step = false;
            } else {
                let regs = cpu.get_regs();
                print!("-> ");
                self.disasm.disasemble(cpu, bus);
                let cmd = self.get_cmd();

                match cmd {
                    Cmd::Break { bkpt} => self.insert_bkpt(bkpt),
                    Cmd::Disassemble { start, end } =>
                        self.print_instr(bus, start, end),
                    Cmd::Error    => { println!("Error"); process::exit(1) },
                    Cmd::Overview => println!("{}", cpu),
                    Cmd::Quit     => { println!("Ta.."); process::exit(0) },
                    Cmd::Run      => run = true,
                    Cmd::Step     => step = true,
                    Cmd::Trace    => trace = true,
                    Cmd::Untrace  => trace = false,
                    Cmd::Unknown  => println!("cmd not known"),
                    Cmd::ViewStack => self.view_stack(bus,
                                                      stack_start,
                                                      regs.gpr[15]),
                    Cmd::View {start, end} => self.print_mem(bus, start, end),
                }
                println!();
            }
        }
    }
}
