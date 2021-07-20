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

pub fn asnat(mut n: usize) -> Vec<Base> {
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

pub fn consts(context: &mut Context) -> Vec<Base> {
    use Base::*;
    let mut result = Vec::new();
    loop {
        match context.dna.as_slice() {
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

pub fn quote(dna: &[Base]) -> Vec<Base> {
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

pub fn protect(l: usize, dna: &[Base]) -> Vec<Base> {
    let mut result = dna.to_vec();
    for _ in 0..l {
        result = quote(&result)
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_quote_unquote(s: &str) {
        let origin = Dna::from_string(s).unwrap();
        let quoted = quote(origin.as_slice());
        let mut context = Context::new(quoted.as_slice().into());
        let transformed = consts(&mut context);
        assert_eq!(origin.as_slice(), transformed.as_slice());
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
        let mut context = Context::new(origin.as_slice().into());
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