use std::path::PathBuf;
use std::time::Instant;

use crate::image::DrawCommand;
use crate::interpreterv2::dna::Dna;
use crate::interpreterv2::interpreter::{Context, do_all_steps, do_step};
use crate::utils::{load, store};

pub fn run_with_logs(context: &mut Context) {
    let start_at = Instant::now();
    let mut step = 0;
    loop {
        if step % 100 == 0 {
            println!("Step: {} Elapsed: {:?}", step, start_at.elapsed());
        }
        if let Err(err) = do_step(context) {
            println!("Finish with: {:?}", err);
            break;
        }
        if start_at.elapsed().as_secs() > 600 {
            break;
        }
        step += 1;
    }
}

crate::entry_point!("interpreterv2", interpreterv2_main);
fn interpreterv2_main() {
    let folder = std::env::args().nth(2).expect("Not enough arguments");
    let dna_path = ["data", &folder, "dna"].iter().collect::<PathBuf>();
    let dna_str = &std::fs::read_to_string(dna_path).unwrap();
    let dna = Dna::from_string(&dna_str).unwrap();

    let mut context = Context::new(dna);

    run_with_logs(&mut context);

//     store(&context, [&folder, "context.ron"].iter().collect::<PathBuf>());

    let commands: Vec<_> = context.rna.iter().filter_map(|dna| {
        DrawCommand::decode(dna)
    }).collect();
    store(&commands, [&folder, "commands.ron"].iter().collect::<PathBuf>());
}

fn produce_draw_commands(dna: Dna) -> Vec<DrawCommand> {
    let mut context = Context::new(dna);
    do_all_steps(&mut context);
    return context.draw_commands();
}

#[test]
fn health_check_test() {
    let health_check_dna = Dna::from_string(&std::fs::read_to_string("data/health_check/dna").unwrap()).unwrap();
    let actual_commands = produce_draw_commands(health_check_dna);
    let expected_commands: Vec<DrawCommand> = load("health_check/commands.ron");
    assert_eq!(expected_commands, actual_commands);
}