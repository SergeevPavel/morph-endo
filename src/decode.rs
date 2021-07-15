use serde::{Deserialize, Serialize};

use crate::dna::{Dna, Base, Subseq};
use crate::literals::{consts, quote, nat, asnat};
use crate::image::DrawCommand;
use std::time::Instant;

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    pub dna: Dna,
    pub rna: Vec<Dna>,
    pub finished: bool,
    pub finish_reason: Vec<String>,
}

impl Context {
    fn append_rna(&mut self, rna: Dna) {
        println!("Draw command: {:?}", DrawCommand::decode(&rna));
        self.rna.push(rna);
    }
}

impl Context {
    pub fn new(dna: Dna) -> Self {
        Context {
            dna,
            rna: vec![],
            finished: false,
            finish_reason: vec![]
        }
    }
}

pub fn do_step(context: &mut Context) -> Option<()> {
    let p = pattern(context)?;
    let t = template(context)?;
    matchreplace(context, p, t);
    return Some(());
}

pub fn execute(context: &mut Context) {
    let stat_moment = Instant::now();
    let mut step = 0;
    loop {
        if let None = do_step(context) {
            println!("Finish with: {:?}", context.finish_reason);
        }
        if context.finished || stat_moment.elapsed().as_secs() > 300 { break }
        step += 1;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PItem {
    PBase(Base),
    Skip { n: usize },
    Search { s: Dna },
    Open,
    Close,
}

type Pattern = Vec<PItem>;

fn pattern(context: &mut Context) -> Option<Pattern> {
    use Base::*;
    let mut p: Pattern = vec![];
    let mut lvl = 0;
    loop {
        match context.dna.data.as_slice() {
            [C, ..] => {
                context.dna = context.dna.subseq(1..);
                p.push(PItem::PBase(I));
            }
            [F, ..] => {
                context.dna = context.dna.subseq(1..);
                p.push(PItem::PBase(C));
            }
            [P, ..] => {
                context.dna = context.dna.subseq(1..);
                p.push(PItem::PBase(F));
            }
            [I, C, ..] => {
                context.dna = context.dna.subseq(2..);
                p.push(PItem::PBase(P));
            }
            [I, P, ..] => {
                context.dna = context.dna.subseq(2..);
                if let Some(n) = nat(context) {
                    p.push(PItem::Skip { n });
                } else {
                    return None;
                }
            }   
            [I, F, ..] => {
                context.dna = context.dna.subseq(3..);
                let s = consts(context);
                p.push(PItem::Search { s });
            }
            [I, I, P, ..] => {
                context.dna = context.dna.subseq(3..);
                lvl += 1;
                p.push(PItem::Open);
            }
            [I, I, C, ..] | [I, I, F, ..] => {
                context.dna = context.dna.subseq(3..);
                if lvl == 0 {
                    return Some(p);
                } else {
                    lvl -= 1;
                    p.push(PItem::Close);
                }
            }
            [I, I, I, ..] => {
                context.append_rna(context.dna.subseq(3..10));
                context.dna = context.dna.subseq(10..)
            }
            dna_tail => {
                context.finish_reason
                    .push(format!("Unexpected dna when pattern decoding {:?}", dna_tail).to_string());
                context.finished = true;
                return None;
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TItem {
    TBase(Base),
    Ref { n: usize, l: usize },
    Len { n: usize }
}

type Template = Vec<TItem>;

fn template(context: &mut Context) -> Option<Template> {
    use TItem::*;
    use Base::*;
    let mut template: Template = vec![];
    loop {
        match context.dna.data.as_slice() {
            [C, ..] => {
                context.dna = context.dna.subseq(1..);
                template.push(TBase(I));
            }
            [F, ..] => {
                context.dna = context.dna.subseq(1..);
                template.push(TBase(C));
            }
            [P, ..] => {
                context.dna = context.dna.subseq(1..);
                template.push(TBase(F));
            }
            [I, C, ..] => {
                context.dna = context.dna.subseq(2..);
                template.push(TBase(P));
            }
            [I, F, ..] | [I, P, ..] => {
                context.dna = context.dna.subseq(2..);
                let l = nat(context)?;
                let n = nat(context)?;
                template.push(Ref { n, l });
            }
            [I, I, C, ..] | [I, I, F, ..] => {
                context.dna = context.dna.subseq(3..);
                return Some(template);
            }
            [I, I, P, ..] => {
                context.dna = context.dna.subseq(3..);
                let n = nat(context)?;
                template.push(Len { n });
            }
            [I, I, I, ..] => {
                context.append_rna(context.dna.subseq(3..10));
                context.dna = context.dna.subseq(10..);
            }
            dna_tail => {
                context.finished = true;
                context.finish_reason
                    .push(format!("Unexpected dna when template decoding {:?}", dna_tail).to_string());
                return None;
            }
        }
    }
}

type Environment = Vec<Dna>;

fn find_subseq(source: &[Base], target: &[Base]) -> Option<usize> {
    source.windows(target.len()).position(|window| window == target).map(|pos| pos + target.len())
}

fn matchreplace(context: &mut Context, pat: Pattern, template: Template) {
    println!("Pattern: {:?} Template: {:?}", pat, template);
    let mut i: usize = 0;
    let mut env: Environment = vec![];
    //c is reversed
    let mut c: Vec<usize> = vec![];
    for p in pat {
        match p {
            PItem::PBase(b) => {
                if context.dna.nth(i) == Some(b) {
                    i += 1;
                } else {
                    return
                }
            },
            PItem::Skip { n } => {
                i += n;
                if i > context.dna.len() {
                    return
                }
            },
            PItem::Search { s } => {
                // todo handle errors in subslicing
                if let Some(n) = find_subseq(&context.dna.data[i..], s.data.as_slice()) {
                    i += n;
                } else {
                    return;
                }
                
            },
            PItem::Open => {
                c.push(i);
            },
            PItem::Close => {
                env.push(context.dna.subseq(c.pop().expect("Unexpectedly empty stack")..i));
            },
        }
    }
    context.dna = context.dna.subseq(i..);
    replace(context, template, env);
}

fn replace(context: &mut Context, template: Template, env: Environment) {
    let mut r = Dna::empty();
    for t in template {
        match t {
            TItem::TBase(b) => r.app(b),
            TItem::Ref { n, l } => {
//                let v = env.get(n).expect(&format!("Out of range! n: {:?} env: {:?}", n, env));
                let v = env.get(n).cloned().unwrap_or(Dna::empty());
                r.concat(&protect(l, v))
            },
            TItem::Len { n } => {
//                let v = env.get(n).expect(&format!("Out of range! n: {:?} env: {:?}", n, env));
                let v = env.get(n).map(|d| d.len()).unwrap_or(0);
                r.concat(&asnat(v))
            },
        }
    }
    r.concat(&context.dna);
    context.dna = r;
}

// rewrite without recursion
fn protect(l: usize, dna: Dna) -> Dna {
    return if l == 0 {
        dna
    } else {
        protect(l - 1, quote(dna))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Base::*;
    
    #[test]
    fn slice_pattern_test() {
        let icfp = Dna::from_string("ICFP").unwrap();
        let branch = match icfp.data.as_slice() {
            [C, ..] => 1,
            [I] => 2,
            [I, ..] => 3,
            _ => 4
        };
        assert_eq!(branch, 3);
    }

    fn pattern_by_str(s: &str) -> Pattern {
        let mut context = Context::new(Dna::from_string(s).unwrap());
        pattern(&mut context).unwrap()
    }

    #[test]
    fn pattern_test() {
        use PItem::*;
        assert_eq!(pattern_by_str("CIIC"), vec![PBase(I)]);
        assert_eq!(pattern_by_str("IIPIPICPIICICIIF"),
                   vec![Open, Skip { n: 2 }, Close, PBase(P)]);
    }

    fn do_step_by_str(s: &str) -> Dna {
        let mut context = Context::new(Dna::from_string(s).unwrap());
        do_step(&mut context).expect("Step failed");
        context.dna
    }

    #[test]
    fn do_step_test() {
        assert_eq!(do_step_by_str("IIPIPICPIICICIIFICCIFPPIICCFPC"),
                   Dna::from_string("PICFC").unwrap());
        assert_eq!(do_step_by_str("IIPIPICPIICICIIFICCIFCCCPPIICCFPC"),
                   Dna::from_string("PIICCFCFFPC").unwrap());
        assert_eq!(do_step_by_str("IIPIPIICPIICIICCIICFCFC"),
                   Dna::from_string("I").unwrap());
    }
}