use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

use crate::interpreter::interpreter::{Context};
use crate::interpreter::pattern::{Pattern, PItem};
use crate::interpreter::template::{Template, TItem};

use super::dna::{Base, Dna};
use super::literals::{asnat, protect};


pub struct Environment(Vec<Dna>);

impl Environment {
    fn get(&self, n: usize) -> Option<&Dna> {
        self.0.get(n)
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (id, dna) in self.0.iter().enumerate() {
            if dna.len() < 1000 {
                writeln!(f, "{:?}: [len: {:?}] {:?}", id, dna.len(), dna)?;
            } else {
                writeln!(f, "{:?}: [len: {:?}] {:?}...", id, dna.len(), dna.subseq(0..1000))?;
            }
        }
        Ok(())
    }
}

fn find_subseq(source: impl Iterator<Item = Base>, target: &[Base]) -> Option<usize> {
    assert!(target.len() > 0);
    let mut window = VecDeque::with_capacity(target.len());
    for (idx, b) in source.enumerate() {
        if window.len() >= target.len() {
            window.pop_front();
        }
        window.push_back(b);
        window.make_contiguous();
        if window.as_slices().0 == target {
            return Some(idx + 1);
        }
    }
    return None;
}

pub fn match_pat(context: &mut Context, pat: Pattern) -> Option<Environment> {
    let mut i: usize = 0;
    let mut env = vec![];
    //c is reversed
    let mut c: Vec<usize> = vec![];
    for p in pat {
        match p {
            PItem::PBase(b) => {
                if context.dna.nth(i) == Some(b) {
                    i += 1;
                } else {
                    return None;
                }
            }
            PItem::Skip { n } => {
                i += n;
                if i > context.dna.len() {
                    return None;
                }
            }
            PItem::Search { s } => {
                if let Some(n) = find_subseq(context.dna.into_iter().skip(i).cloned(), s.as_slice()) {
                    i += n;
                } else {
                    return None;
                }
            }
            PItem::Open => {
                c.push(i);
            }
            PItem::Close => {
                env.push(
                    context
                        .dna
                        .subseq(c.pop().expect("Unexpectedly empty stack")..i),
                );
            }
        }
    }
    context.dna.skip(i);
    return Some(Environment(env));
}

pub fn replace(context: &mut Context, template: Template, env: Environment) {
    let mut r = Dna::empty();
    for t in template {
        match t {
            TItem::TBase(b) => {
                r = r.concat(&Dna::from_slice(&[b]));
            }
            TItem::Ref { n, l } => {
                if let Some(v) = env.get(n) {
                    r = r.concat(&protect(l, v))
                }
            }
            TItem::Len { n } => {
                let v = env.get(n).map(|d| d.len()).unwrap_or(0);
                r = r.concat(&Dna::from_slice(&asnat(v)))
            }
        }
    }
    r = r.concat(&context.dna);
    context.dna = r;
}