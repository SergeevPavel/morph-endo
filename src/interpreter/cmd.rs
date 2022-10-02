use std::io::{Read, stdin, stdout, Write};
use std::path::Path;
use crate::interpreter::dna::Dna;

use crate::interpreter::interpreter::Context;
use crate::interpreter::match_replace::{match_pat, replace};
use crate::interpreter::pattern::pattern;
use crate::interpreter::template::template;

fn read_dna<P: AsRef<Path>>(path: P) -> Dna {
    let dna_str = &std::fs::read_to_string(path).unwrap();
    return Dna::from_string(&dna_str).unwrap();
}

fn dna_for_prefix(dna_prefix: &str) -> Dna {
    let prefix_dna = Dna::from_string(&dna_prefix).unwrap();
    let endo_dna = read_dna("data/endo/dna");
    return prefix_dna.concat(&endo_dna);
}

crate::entry_point!("cmd", interpreter_cmd);
fn interpreter_cmd() {
    println!("Please enter DNA prefix:");
    let mut dna_prefix = String::new();
    stdin().read_line(&mut dna_prefix).unwrap();
    println!("Prefix: {:?}", dna_prefix);

    let mut context = Context::new(dna_for_prefix(&dna_prefix.trim()));

    loop {
        let p = pattern(&mut context).unwrap();
        println!("Pat: {:?}", p);
        let t = template(&mut context).unwrap();
        println!("Tmp: {:?}", t);
        if let Some(env) = match_pat(&mut context, p) {
            println!("Env:\n{:?}", env);
            replace(&mut context, t, env);
        }
        stdin().read(&mut [0u8]).unwrap();
    }

}