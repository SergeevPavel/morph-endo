use std::time::Instant;

use crate::interpreterv2::dna::{Dna, ShortDna};

use super::{pattern::pattern, template::template, match_replace::match_pat};
use crate::interpreterv2::match_replace::replace;

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
}

pub fn do_step(context: &mut Context) -> InterpreterResult<()> {
    let p = pattern(context)?;
    let t = template(context)?;
    if let Some(env) = match_pat(context, p) {
        replace(context, t, env);
    }
    return Ok(());
}

pub fn execute(context: &mut Context) {
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