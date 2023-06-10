use std::fmt::{Debug, Formatter, Write};
use crate::interpreter::dna::Base;
use crate::interpreter::interpreter::{Context, InterpreterResult};
use crate::interpreter::literals::*;

use super::dna::ShortDna;


#[derive(Clone, Eq, PartialEq)]
pub enum TItem {
    TBase(Base),
    Ref { n: usize, l: usize },
    Len { n: usize },
}

impl Debug for TItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TItem::TBase(b) => {
                f.write_char(b.to_char())
            }
            TItem::Ref { n, l } => {
                write!(f, "Ref(num={:?}, prot_lvl={:?})", n, l)
            }
            TItem::Len { n } => {
                write!(f, "Len({:?})", n)
            }
        }
    }
}

impl TItem {
    pub fn encode(&self) -> ShortDna {
        use Base::*;
        let mut result = Vec::new();
        match self {
            TItem::TBase(b) => match b {
                Base::I => result.push(C),
                Base::C => result.push(F),
                Base::F => result.push(P),
                Base::P => result.extend_from_slice(&[I, C]),
            },
            TItem::Ref { n, l } => {
                result.extend_from_slice(&[I, F]);
                result.extend_from_slice(&asnat(*n));
                result.extend_from_slice(&asnat(*l));
            },
            TItem::Len { n } => {
                result.extend_from_slice(&[I, I, P]);
                result.extend_from_slice(&asnat(*n));
            },
        }
        return result;
    }
}

pub type Template = Vec<TItem>;

pub fn encode(t: &Template) -> ShortDna {
    t.iter().flat_map(|t| t.encode()).collect()
}

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
