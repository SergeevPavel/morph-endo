use crate::dna::*;
use crate::decode::{Context, execute};
use crate::utils::store;
use crate::image::DrawCommand;
use std::path::PathBuf;

crate::entry_point!("interpreter", interpreter_main);
fn interpreter_main() {
    let folder = std::env::args().nth(2).expect("Not enough arguments");
    let dna_path = ["data", &folder, "dna"].iter().collect::<PathBuf>();
    let dna_str = &std::fs::read_to_string(dna_path).unwrap();
    let dna = Dna::from_string(dna_str).unwrap();
    let mut context = Context::new(dna);
    execute(&mut context);

    store(&context, [&folder, "context.ron"].iter().collect::<PathBuf>());

    let commands: Vec<_> = context.rna.iter().filter_map(|dna| {
        let command = DrawCommand::decode(dna);
        if command.is_none() {
            println!("Dna {:?}", dna);
        }
        command
    }).collect();
    store(&commands, [&folder, "commands.ron"].iter().collect::<PathBuf>());
}

