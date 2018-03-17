/*
 * TODO: fix the code in main()
 */

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod bf;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: bf <file>");
        return;
    }

    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let ir = bf::parse(&contents);
    bf::run(&ir);
}