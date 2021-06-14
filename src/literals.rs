use crate::dna::{Dna, Base, Subseq};
use crate::decode::{Context};

// TODO rewrite without recursion
pub fn nat(context: &mut Context) -> Option<usize> {
    use Base::*;
    return match context.dna.data.as_slice() {
        [P, ..] => {
            context.dna = context.dna.subseq(1..);
            Some(0)
        }
        [I, ..] | [F, ..] => {
            context.dna = context.dna.subseq(1..);
            nat(context).map(|n| n * 2)
        }
        [C, ..] => {
            context.dna = context.dna.subseq(1..);
            nat(context).map(|n| n * 2 + 1)
        }
        _ => {
            context.finished = true;
            None
        }
    }
}

// rewriten wihout recursion
pub fn asnat(mut n: usize) -> Dna {
    use Base::*;
    let mut result = Dna::empty();
    while n != 0 {
        if n % 2 == 0 {
            result.app(I);
        } else {
            result.app(C);
        }
        n /= 2;
    }
    result.app(P);
    result
}

// TODO rewrite without recursion
pub fn consts(context: &mut Context) -> Dna {
    use Base::*;
    return match context.dna.data.as_slice() {
        [C, ..] => {
            context.dna = context.dna.subseq(1..);
            let mut s = consts(context);
            s.prep(I);
            s
        }
        [F, ..] => {
            context.dna = context.dna.subseq(1..);
            let mut s = consts(context);
            s.prep(C);
            s
        }
        [P, ..] => {
            context.dna = context.dna.subseq(1..);
            let mut s = consts(context);
            s.prep(F);
            s
        }
        [I, C, ..] => {
            context.dna = context.dna.subseq(2..);
            let mut s = consts(context);
            s.prep(P);
            s
        }
        _ => Dna::empty()
    }
}

// rewriten wihout recursion
pub fn quote(dna: Dna) -> Dna {
    use Base::*;
    let mut result = Dna::empty();
    for b in dna.data {
        match b {
            I => result.app(C),
            C => result.app(F),
            F => result.app(P),
            P => {
                result.app(I);
                result.app(C);
            },
        }
    }
    return result;
}