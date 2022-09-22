use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::image::DrawCommand;
use crate::interpreter::dna::Dna;
use crate::interpreter::interpreter::{Context, do_all_steps, do_step};
use crate::utils::{load, store};

pub fn run_with_logs(context: &mut Context) {
    let start_at = Instant::now();
    let mut step = 0;
    loop {
        if step % 10000 == 0 {
            println!("Step: {} Elapsed: {:?}", step, start_at.elapsed());
        }
        if let Err(err) = do_step(context) {
            println!("Finish with: {:?} on {:?}", err, step);
            break;
        }
        if start_at.elapsed().as_secs() > 600 {
            break;
        }
        step += 1;
    }
}

fn read_dna<P: AsRef<Path>>(path: P) -> Dna {
    let dna_str = &std::fs::read_to_string(path).unwrap();
    return Dna::from_string(&dna_str).unwrap();
}

fn dna_for_task<S: AsRef<str>>(task: S) -> Dna {
    let endo_dna = read_dna("data/endo/dna");
    let prefix_dna = read_dna(["data", task.as_ref(), "dna"].iter().collect::<PathBuf>());
    return prefix_dna.concat(&endo_dna);
}

crate::entry_point!("interpreter", interpreter_main);
fn interpreter_main() {
    let task = std::env::args().nth(2).expect("Not enough arguments");
    println!("Run interpreter on {}", task);
    let mut context = Context::new(dna_for_task(&task));
    run_with_logs(&mut context);

//     store(&context, [&folder, "context.ron"].iter().collect::<PathBuf>());

    println!("Produced: {} operations", context.rna.len());
    let commands: Vec<_> = context.rna.iter().filter_map(|dna| {
        DrawCommand::decode(dna)
    }).collect();
    println!("Valid: {} commands", commands.len());
    store(&commands, [&task, "commands.ron"].iter().collect::<PathBuf>());
}

fn produce_draw_commands(dna: Dna) -> Vec<DrawCommand> {
    let mut context = Context::new(dna);
    do_all_steps(&mut context);
    return context.draw_commands();
}

fn check_for<P: AsRef<str>, S: AsRef<str>>(task: P, task_name: S) {
    let dna = dna_for_task(&task);
    let start_time = Instant::now();
    let actual_commands = produce_draw_commands(dna);
    println!("{} took: {:?}", task_name.as_ref(), start_time.elapsed());
    let expected_commands: Vec<DrawCommand> = load(["data", task.as_ref(), "commands.ron"].iter().collect::<PathBuf>());
    assert_eq!(expected_commands, actual_commands);
}

#[test]
fn health_check_test() {
    check_for("health_check", "Health check");
    check_for("repair_guide/initial", "Repair guide");
}

#[test]
fn bench() {
    //65.00281809s
    check_for("repair_topics", "Repair topics");
}