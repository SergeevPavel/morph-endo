use crate::dna::{Dna, Base};
use crate::decode::{Context};

// recursion
// is it possible to overflow usize?
pub fn nat(context: &mut Context) -> Option<usize> {
    use Base::*;
    return match context.dna.as_slice() {
        [P, ..] => {
            context.dna.skip(1);
            Some(0)
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
            context.finished = true;
            context.finish_reason
                .push(format!("Unexpected dna when nat decoding {:?}", dna_tail).to_string());
            None
        }
    }
}

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

pub fn consts(context: &mut Context) -> Dna {
    use Base::*;
    let mut result = Dna::empty();
    loop {
        match context.dna.as_slice() {
            [C, ..] => {
                context.dna.skip(1);
                result.app(I);
            }
            [F, ..] => {
                context.dna.skip(1);
                result.app(C);
            }
            [P, ..] => {
                context.dna.skip(1);
                result.app(F);
            }
            [I, C, ..] => {
                context.dna.skip(2);
                result.app(P);
            }
            _ => break
        }
    }
    return result;
}

pub fn quote(dna: Dna) -> Dna {
    use Base::*;
    let mut result = Dna::empty();
    for b in dna.as_slice() {
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

pub fn protect(l: usize, mut dna: Dna) -> Dna {
    for _ in 0..l {
        dna = quote(dna)
    }
    dna
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