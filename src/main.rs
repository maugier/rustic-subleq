extern crate clap;
extern crate rand;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;

mod vm;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("Rustic Subleq")
        .version("0.1")
        .arg(Arg::with_name("memsz")
                 .short("m")
                 .value_name("SIZE")
                 .help("Change the virtual memory size"))
        .arg(Arg::with_name("FILE")
                 .required(true)
                 .help("Bytecode to load"))
        .get_matches();

    let mut text = Vec::new();
    File::open(matches.value_of("FILE").unwrap())?.read_to_end(&mut text)?;

    let mut cpu = vm::CPU::new(vm::Bus::new(text));

    cpu.run()

}
