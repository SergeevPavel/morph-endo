#![allow(dead_code)]

mod dna;
mod decode;
mod literals;

use dna::*;
use decode::{Context, execute};

fn main() {
    let dna = Dna::from_string(&std::fs::read_to_string("data/endo.dna").unwrap()).unwrap();
    let mut context = Context::new(dna);
    execute(&mut context);
    println!("{:?}", context);
}


