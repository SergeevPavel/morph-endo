use std::path::{PathBuf};
use std::time::Instant;

use crate::image::DrawCommand;
use crate::interpreter::dna::Dna;
use crate::interpreter::interpreter::{Context, do_all_steps, do_step};
use crate::utils::{store};

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

crate::entry_point!("interpreter", interpreter_main);
fn interpreter_main() {
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
    fn check_for<P: AsRef<Path>, S: AsRef<str>>(example_path: P, example_name: S) {
        let health_check_dna = Dna::from_string(&std::fs::read_to_string(example_path.as_ref().join("dna")).unwrap()).unwrap();
        let start_time = Instant::now();
        let actual_commands = produce_draw_commands(health_check_dna);
        println!("{} took: {:?}", example_name.as_ref(), start_time.elapsed());
        let expected_commands: Vec<DrawCommand> = load(example_path.as_ref().join("commands.ron"));
        assert_eq!(expected_commands, actual_commands);
    }
    check_for("data/health_check", "Health check");
    check_for("data/repair_guide", "Repair guide");
}