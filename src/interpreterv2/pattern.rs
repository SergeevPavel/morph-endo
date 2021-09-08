use crate::interpreterv2::dna::Base;
use crate::interpreterv2::interpreter::{Context, InterpreterResult};
use crate::interpreterv2::literals::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PItem {
    PBase(Base),
    Skip { n: usize },
    Search { s: Vec<Base> },
    Open,
    Close,
}

pub type Pattern = Vec<PItem>;

 pub fn pattern(context: &mut Context) -> InterpreterResult<Pattern> {
     use Base::*;
     let mut p: Pattern = vec![];
     let mut lvl = 0;
     loop {
         match context.dna.prefix(3).as_slice() {
             [C, ..] => {
                 context.dna.skip(1);
                 p.push(PItem::PBase(I));
             }
             [F, ..] => {
                 context.dna.skip(1);
                 p.push(PItem::PBase(C));
             }
             [P, ..] => {
                 context.dna.skip(1);
                 p.push(PItem::PBase(F));
             }
             [I, C, ..] => {
                 context.dna.skip(2);
                 p.push(PItem::PBase(P));
             }
             [I, P, ..] => {
                 context.dna.skip(2);
                 p.push(PItem::Skip { n: nat(context)? });
             }
             [I, F, ..] => {
                 context.dna.skip(3);
                 let s = consts(context);
                 p.push(PItem::Search { s });
             }
             [I, I, P, ..] => {
                 context.dna.skip(3);
                 lvl += 1;
                 p.push(PItem::Open);
             }
             [I, I, C, ..] | [I, I, F, ..] => {
                 context.dna.skip(3);
                 if lvl == 0 {
                     return Ok(p);
                 } else {
                     lvl -= 1;
                     p.push(PItem::Close);
                 }
             }
             [I, I, I, ..] => {
                 context.append_rna(context.dna.to_vec(3..10));
                 context.dna.skip(10);
             }
             dna_tail => {
                 return Err(format!("Unexpected dna when pattern decoding {:?}", dna_tail).to_string())
             }
         }
     }
 }
