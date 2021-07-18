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
        dna_tail => {
            context.finished = true;
            context.finish_reason
                .push(format!("Unexpected dna when nat decoding {:?}", dna_tail).to_string());
            None
        }
    }
}

// rewritten without recursion
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

#[cfg(test)]
mod tests {
    use super::*;

    fn check_quote_unquote(s: &str) {
        let origin = Dna::from_string(s).unwrap();
        let quoted = quote(origin.clone());
        let mut context = Context::new(quoted);
        let transformed = consts(&mut context);
        assert_eq!(origin, transformed);
        assert_eq!(context.dna.len(), 0);
    }

    #[test]
    fn const_test() {
        check_quote_unquote("IIPIPICPIICICIIFICCIFPPIICCFPC");
        check_quote_unquote("IIPIPICPIICICIIFICCIFCCCPPIICCFPC");
        check_quote_unquote("IIPIPIICPIICIICCIICFCFC");
    }

    fn check_asnat_nat(n: usize) {
        let origin = asnat(n);
        let mut context = Context::new(origin);
        let transformed = nat(&mut context);
        assert_eq!(Some(n), transformed);
        assert_eq!(context.dna.len(), 0);
    }

    #[test]
    fn num_test() {
        check_asnat_nat(42);
        check_asnat_nat(100);
        check_asnat_nat(0);
        check_asnat_nat(1);
    }
}