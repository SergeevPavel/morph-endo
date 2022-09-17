use crate::interpreter::dna::Base;
use crate::interpreter::interpreter::{Context, InterpreterResult};
use crate::interpreter::literals::*;


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TItem {
    TBase(Base),
    Ref { n: usize, l: usize },
    Len { n: usize },
}

pub type Template = Vec<TItem>;

pub fn template(context: &mut Context) -> InterpreterResult<Template> {
    use Base::*;
    use TItem::*;
    let mut template: Template = vec![];
    loop {
        match context.dna.prefix(3).as_slice() {
            [C, ..] => {
                context.dna.skip(1);
                template.push(TBase(I));
            }
            [F, ..] => {
                context.dna.skip(1);
                template.push(TBase(C));
            }
            [P, ..] => {
                context.dna.skip(1);
                template.push(TBase(F));
            }
            [I, C, ..] => {
                context.dna.skip(2);
                template.push(TBase(P));
            }
            [I, F, ..] | [I, P, ..] => {
                context.dna.skip(2);
                let l = nat(context)?;
                let n = nat(context)?;
                template.push(Ref { n, l });
            }
            [I, I, C, ..] | [I, I, F, ..] => {
                context.dna.skip(3);
                return Ok(template);
            }
            [I, I, P, ..] => {
                context.dna.skip(3);
                let n = nat(context)?;
                template.push(Len { n });
            }
            [I, I, I, ..] => {
                context.append_rna(context.dna.to_vec(3..10));
                context.dna.skip(10);
            }
            dna_tail => {
                return Err(format!("Unexpected dna when template decoding {:?}", dna_tail).to_string());
            }
        }
    }
}
