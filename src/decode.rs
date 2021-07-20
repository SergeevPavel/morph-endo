use serde::{Deserialize, Serialize};

use crate::dna::{Base, Dna, Subseq};
use crate::literals::{asnat, consts, nat, protect};
use std::time::Instant;
use std::collections::VecDeque;
// use crate::image::DrawCommand;

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    pub dna: Dna,
    pub rna: Vec<Dna>,
    pub finished: bool,
    pub finish_reason: Vec<String>,
}

impl Context {
    fn append_rna(&mut self, rna: Dna) {
        // println!("Draw command: {:?}", DrawCommand::decode(&rna));
        self.rna.push(rna);
    }
}

impl Context {
    pub fn new(dna: Dna) -> Self {
        Context {
            dna,
            rna: vec![],
            finished: false,
            finish_reason: vec![],
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
    let start_at = Instant::now();
    let mut step = 0;
    loop {
        if step % 100 == 0 {
            println!("Step: {} Elapsed: {:?}", step, start_at.elapsed());
        }
        if let None = do_step(context) {
            println!("Finish with: {:?}", context.finish_reason);
        }
        if context.finished || start_at.elapsed().as_secs() > 600 {
            break;
        }
        step += 1;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PItem {
    PBase(Base),
    Skip { n: usize },
    Search { s: Vec<Base> },
    Open,
    Close,
}

type Pattern = Vec<PItem>;

fn pattern(context: &mut Context) -> Option<Pattern> {
    use Base::*;
    let mut p: Pattern = vec![];
    let mut lvl = 0;
    loop {
        match context.dna.as_slice() {
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
                if let Some(n) = nat(context) {
                    p.push(PItem::Skip { n });
                } else {
                    return None;
                }
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
                    return Some(p);
                } else {
                    lvl -= 1;
                    p.push(PItem::Close);
                }
            }
            [I, I, I, ..] => {
                context.append_rna(context.dna.subseq(3..10));
                context.dna.skip(10);
            }
            dna_tail => {
                context.finish_reason.push(
                    format!("Unexpected dna when pattern decoding {:?}", dna_tail).to_string(),
                );
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
    Len { n: usize },
}

type Template = Vec<TItem>;

fn template(context: &mut Context) -> Option<Template> {
    use Base::*;
    use TItem::*;
    let mut template: Template = vec![];
    loop {
        match context.dna.as_slice() {
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
                return Some(template);
            }
            [I, I, P, ..] => {
                context.dna.skip(3);
                let n = nat(context)?;
                template.push(Len { n });
            }
            [I, I, I, ..] => {
                context.append_rna(context.dna.subseq(3..10));
                context.dna.skip(10);
            }
            dna_tail => {
                context.finished = true;
                context.finish_reason.push(
                    format!("Unexpected dna when template decoding {:?}", dna_tail).to_string(),
                );
                return None;
            }
        }
    }
}

type Environment = Vec<Dna>;

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

fn matchreplace(context: &mut Context, pat: Pattern, template: Template) {
    // println!("Pattern: {:?} Template: {:?}", pat, template);
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
                    return;
                }
            }
            PItem::Skip { n } => {
                i += n;
                if i > context.dna.len() {
                    return;
                }
            }
            PItem::Search { s } => {
                if let Some(n) = find_subseq(context.dna.iter().skip(i), s.as_slice()) {
                    i += n;
                } else {
                    return;
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
    replace(context, template, env);
}

fn replace(context: &mut Context, template: Template, env: Environment) {
    let mut r = Dna::empty();
    for t in template {
        match t {
            TItem::TBase(b) => r.app(b),
            TItem::Ref { n, l } => {
                // let v = env.get(n).expect(&format!("Out of range! n: {:?} env: {:?}", n, env));
                let v = env.get(n).cloned().unwrap_or(Dna::empty());
                r.concat(&protect(l, v.as_slice()).as_slice().into())
            }
            TItem::Len { n } => {
                // let v = env.get(n).expect(&format!("Out of range! n: {:?} env: {:?}", n, env));
                let v = env.get(n).map(|d| d.len()).unwrap_or(0);
                r.concat(&asnat(v).as_slice().into())
            }
        }
    }
    r.concat(&context.dna);
    context.dna = r;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::{PathBuf, Path};
    use Base::*;

    #[test]
    fn slice_pattern_test() {
        let icfp = Dna::from_string("ICFP").unwrap();
        let branch = match icfp.as_slice() {
            [C, ..] => 1,
            [I] => 2,
            [I, ..] => 3,
            _ => 4,
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
        assert_eq!(
            pattern_by_str("IIPIPICPIICICIIF"),
            vec![Open, Skip { n: 2 }, Close, PBase(P)]
        );
    }

    fn do_step_by_str(s: &str) -> Dna {
        let mut context = Context::new(Dna::from_string(s).unwrap());
        do_step(&mut context).expect("Step failed");
        context.dna
    }

    #[test]
    fn find_subseq_test() {
        use Base::*;
        assert_eq!(find_subseq([I, C, F, P, I, C, F, P].iter().cloned(),
                               &[F, P, I]),
                   Some(5));
        assert_eq!(find_subseq([I, C, F, P, I, C, F, P].iter().cloned(),
                               &[F, F, I]),
                   None);
        assert_eq!(find_subseq([I, C, F, P, I, C, F, P].iter().cloned(),
                               &[F, P, C]),
                   None);
        assert_eq!(find_subseq([I, C, F, P, I, C, F, P].iter().cloned(),
                               &[F, I, C]),
                   None);
        assert_eq!(find_subseq([I, C, F, P, I, C, F, P].iter().cloned(),
                               &[I, C]),
                   Some(2));
        assert_eq!(find_subseq([I, C, F, P, I, C, F, P].iter().cloned(),
                               &[I, C, F, P, I, C, F, P]),
                   Some(8));
    }

    #[test]
    fn do_step_test() {
        assert_eq!(
            do_step_by_str("IIPIPICPIICICIIFICCIFPPIICCFPC"),
            Dna::from_string("PICFC").unwrap()
        );
        assert_eq!(
            do_step_by_str("IIPIPICPIICICIIFICCIFCCCPPIICCFPC"),
            Dna::from_string("PIICCFCFFPC").unwrap()
        );
        assert_eq!(
            do_step_by_str("IIPIPIICPIICIICCIICFCFC"),
            Dna::from_string("I").unwrap()
        );
    }

    #[test]
    fn multistep_test() {
        #[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
        struct TestData {
            dna: String,
            rna: Vec<String>,
        }
        
        fn serialize<P: AsRef<Path>>(path: P, data: &TestData) {
            let config = ron::ser::PrettyConfig::new().with_depth_limit(4);
            let file = std::fs::File::create(path.as_ref()).unwrap();
            ron::ser::to_writer_pretty(
                    std::io::BufWriter::new(file),
                    &data,
                    config,
            ).unwrap();
        }

        let dna_path: PathBuf = ["data", "tests", "interpreter", "endo.dna"]
            .iter()
            .collect();
        let dna_str = &std::fs::read_to_string(dna_path).unwrap();
        let dna = Dna::from_string(dna_str).unwrap();
        let mut context = Context::new(dna);
        let step_to_iterate = 100;
        let save_every = 10;
        for step in 1..step_to_iterate {
            do_step(&mut context).unwrap();
            if step % save_every == 0 {
                let actual_test_data = TestData {
                    dna: context.dna.to_string(),
                    rna: context.rna.iter().map(|d| d.to_string()).collect(),
                };
                let test_data_path: PathBuf = [
                    "data",
                    "tests",
                    "interpreter",
                    "expected",
                    format!("step_{}", step).as_str(),
                ].iter().collect();
                if test_data_path.exists() {
                    let file = File::open(&test_data_path).unwrap();
                    let expected_test_data: TestData =
                        ron::de::from_reader(std::io::BufReader::new(file)).unwrap();
                    if actual_test_data != expected_test_data {
                        let unexpected_data_path: PathBuf = [
                            "data",
                            "tests",
                            "interpreter",
                            "unexpected",
                            format!("step_{}", step).as_str(),
                        ].iter().collect();
                        serialize(unexpected_data_path, &actual_test_data);
                        panic!("Unexpected test data at step {}!", step);
                    }
                } else {
                    serialize(test_data_path, &actual_test_data);
                }
            }
        }
    }
}
