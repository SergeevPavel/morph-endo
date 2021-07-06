use crate::dna::*;
use crate::decode::{Context, execute};
use crate::utils::store;
use crate::image::DrawCommand;

crate::entry_point!("interpreter", interpreter_main);

fn interpreter_main() {
    let dna = Dna::from_string(&std::fs::read_to_string("data/endo.dna").unwrap()).unwrap();
    let mut context = Context::new(dna);
    execute(&mut context);
    store(&context, "context.ron");

    let commands: Vec<_> = context.rna.iter().filter_map(|dna| {
        let command = DrawCommand::decode(dna);
        if command.is_none() {
            println!("Dna {:?}", dna);
        }
        command
    }).collect();
    store(&commands, "commands.ron");
}

