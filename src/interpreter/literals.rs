use std::mem::swap;
use crate::interpreter::interpreter::{Context, InterpreterResult};
use crate::interpreter::dna::{Base, Dna, ShortDna};

// recursion
// is it possible to overflow usize?
pub fn nat(context: &mut Context) -> InterpreterResult<usize> {
    use Base::*;
    return match context.dna.prefix(1).as_slice() {
        [P, ..] => {
            context.dna.skip(1);
            Ok(0)
        }
        [I, ..] | [F, ..] => {
            context.dna.skip(1);
            nat(context).map(|n| n * 2)
        }
        [C, ..] => {
            context.dna.skip(1);
            nat(context).map(|n| n * 2 + 1)
        }
        dna_tail => {
            return Err(format!("Unexpected dna when nat decoding {:?}", dna_tail).to_string());
        }
    }
}

 pub fn asnat(mut n: usize) -> ShortDna {
     use Base::*;
     let mut result = Vec::new();
     while n != 0 {
         if n % 2 == 0 {
             result.push(I);
         } else {
             result.push(C);
         }
         n /= 2;
     }
     result.push(P);
     result
 }

 pub fn consts(context: &mut Context) -> ShortDna {
     use Base::*;
     let mut result = Vec::new();
     loop {
         match context.dna.prefix(2).as_slice() {
             [C, ..] => {
                 context.dna.skip(1);
                 result.push(I);
             }
             [F, ..] => {
                 context.dna.skip(1);
                 result.push(C);
             }
             [P, ..] => {
                 context.dna.skip(1);
                 result.push(F);
             }
             [I, C, ..] => {
                 context.dna.skip(2);
                 result.push(P);
             }
             _ => break
         }
     }
     return result;
 }

 pub fn protect(l: usize, dna: &Dna) -> Dna {
     if l == 0 { return dna.clone(); }

     let mut result = dna.to_vec(0..dna.len());
     let mut next_vec = Vec::with_capacity(dna.len() * 2);
     for _ in 0..l {
         next_vec.clear();

         use Base::*;
         for b in &result {
             match *b {
                 I => next_vec.push(C),
                 C => next_vec.push(F),
                 F => next_vec.push(P),
                 P => {
                     next_vec.push(I);
                     next_vec.push(C);
                 },
             }
         }
         swap(&mut result, &mut next_vec);
     }
     return Dna::from_slice(&result);
 }

#[test]
fn asnat_test() {
    println!("{:?}", asnat(42));
}