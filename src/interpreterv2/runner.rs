use std::path::PathBuf;
use crate::image::DrawCommand;
use crate::interpreterv2::dna::Dna;
use crate::interpreterv2::interpreter::Context;
use crate::utils::store;

use super::interpreter::execute;



crate::entry_point!("interpreterv2", interpreterv2_main);
fn interpreterv2_main() {
    let folder = std::env::args().nth(2).expect("Not enough arguments");
    let dna_path = ["data", &folder, "dna"].iter().collect::<PathBuf>();
    let dna_str = &std::fs::read_to_string(dna_path).unwrap();
    let dna = Dna::from_string(&dna_str).unwrap();

    let mut context = Context::new(dna);

     execute(&mut context);

//     store(&context, [&folder, "context.ron"].iter().collect::<PathBuf>());

     let commands: Vec<_> = context.rna.iter().filter_map(|dna| {
         DrawCommand::decode(dna)
     }).collect();
     store(&commands, [&folder, "commands.ron"].iter().collect::<PathBuf>());
}