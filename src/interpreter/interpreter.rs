use crate::image::DrawCommand;
use crate::interpreter::dna::{Dna, ShortDna};
use crate::interpreter::match_replace::replace;

use super::{match_replace::match_pat, pattern::pattern, template::template};

pub type InterpreterResult<T> = Result<T, String>;


#[derive(Debug)]
pub struct Context {
    pub dna: Dna,
    pub rna: Vec<ShortDna>,
}

impl Context {
    pub fn new(dna: Dna) -> Self {
        Context {
            dna,
            rna: vec![],
        }
    }

    pub fn append_rna(&mut self, rna: ShortDna) {
        self.rna.push(rna);
    }

    pub fn draw_commands(&self) -> Vec<DrawCommand> {
        self.rna.iter().filter_map(|dna| {
            DrawCommand::decode(dna)
        }).collect()
    }
}

pub fn do_step(context: &mut Context) -> InterpreterResult<()> {
    let p = pattern(context)?;
    let t = template(context)?;
    if let Some(env) = match_pat(context, p) {
        replace(context, t, env);
    }
    return Ok(());
}

pub fn do_all_steps(context: &mut Context) -> String {
    loop {
        if let Err(reason_to_stop) = do_step(context) {
            return reason_to_stop
        }
    }
}