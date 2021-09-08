use crate::interpreterv2::interpreter::{Context, InterpreterResult};
use crate::interpreterv2::dna::{Base, ShortDna};

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

 pub fn quote(dna: &[Base]) -> ShortDna {
     use Base::*;
     let mut result = Vec::new();
     for b in dna {
         match b {
             I => result.push(C),
             C => result.push(F),
             F => result.push(P),
             P => {
                 result.push(I);
                 result.push(C);
             },
         }
     }
     return result;
 }

 pub fn protect(l: usize, dna: &[Base]) -> ShortDna {
     let mut result = dna.to_vec();
     for _ in 0..l {
         result = quote(&result)
     }
     result
 }