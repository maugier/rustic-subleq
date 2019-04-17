extern crate clap;
extern crate rand;

use clap::{Arg, App};

mod vm;

fn main() {
    let matches = App::new("Rustic Subleq")
        .version("0.1")
        .arg(Arg::with_name("memsz")
                 .short("m")
                 .value_name("SIZE")
                 .help("Change the virtual memory size"))
        .arg(Arg::with_name("FILE")
                 .help("Bytecode to load"))
        .get_matches();

    println!("Hello, world!");
}
