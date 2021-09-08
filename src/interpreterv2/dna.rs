use crate::interpreterv2::rope::{Seq};
pub use crate::dna::Base;

pub type ShortDna = Vec<Base>;

#[derive(Clone)]
pub struct Dna {
    pub seq: Seq<Base>
}

impl std::fmt::Debug for Dna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list_repr = f.debug_list();
        for b in self.seq.into_iter() {
            list_repr.entry(b);
        }
        list_repr.finish()
    }
}

impl Dna {
    pub fn empty() -> Self {
        Dna { seq: Seq::from_slice(&[]) }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let data: Result<Vec<Base>, String> = s.chars().map(|c| {
            match c {
                'I' => Ok(Base::I),
                'C' => Ok(Base::C),
                'F' => Ok(Base::F),
                'P' => Ok(Base::P),
                other => Err(format!("Unexpected symbol {}", other).to_string())
            }
        }).collect();
        Ok(Dna{
            seq: Seq::from_slice(data?.as_slice())
        })
    }

    pub fn from_slice(s: &[Base]) -> Self {
        Dna { seq: Seq::from_slice(s) }
    }

    pub fn prefix(&self, size: usize) -> Vec<Base> {
        self.to_vec(0..size)
    }

    pub fn skip(&mut self, count: usize) {
        self.seq = self.seq.subseq(count..);
    }

    pub fn to_vec(&self, range: std::ops::Range<usize>) -> Vec<Base> {
        self.seq.into_iter()
        .skip(range.start)
        .take(range.count())
        .cloned()
        .collect()
    }

    pub fn nth(&self, idx: usize) -> Option<Base> {
        self.seq.nth(idx).cloned()
    }

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn subseq(&self, range: std::ops::Range<usize>) -> Dna {
        Dna { seq: self.seq.subseq(range) }
    }

    pub fn concat(&self, other: &Self) -> Self {
        Dna { seq: self.seq.concat(&other.seq) }
    }
}

