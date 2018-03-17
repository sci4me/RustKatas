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

    let file = File::open(&args[1]);
    if file.is_err() {
        println!("{}", file.err().unwrap());
        return;
    }

    let mut contents = String::new();
    let r = file.unwrap().read_to_string(&mut contents);
    if r.is_err() {
        println!("{}", r.err().unwrap());
        return;
    }

    let ir = bf::parse(&contents);
    bf::run(&ir);
}