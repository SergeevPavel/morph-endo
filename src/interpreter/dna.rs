use std::fmt::Write;
use std::iter::Skip;
use crate::interpreter::rope::{MAX_LEAF, Seq, SeqIter};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Base {
    I, C, F, P
}

impl Default for Base {
    fn default() -> Self {
        Base::I
    }
}

impl Base {
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'I' => Ok(Base::I),
            'C' => Ok(Base::C),
            'F' => Ok(Base::F),
            'P' => Ok(Base::P),
            other => Err(format!("Unexpected symbol {}", other).to_string())
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Base::I => 'I',
            Base::C => 'C',
            Base::F => 'F',
            Base::P => 'P'
        }
    }
}

pub type ShortDna = Vec<Base>;

#[derive(Clone)]
pub struct Dna {
    skipped: usize,
    seq: Seq<Base>
}

impl std::fmt::Debug for Dna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.seq.into_iter() {
            let c = match b {
                Base::I => 'I',
                Base::C => 'C',
                Base::F => 'F',
                Base::P => 'P'
            };
            f.write_char(c)?;
        }
        Ok(())
        // let mut list_repr = f.debug_list();
        // for b in self.seq.into_iter() {
        //     list_repr.entry(b);
        // }
        // list_repr.finish()
    }
}

impl Dna {
    pub fn empty() -> Self {
        Dna { skipped: 0, seq: Seq::from_slice(&[]) }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let data: Result<Vec<Base>, String> = s.chars().map(|c| {
            Base::from_char(c)
        }).collect();
        Ok(Dna{
            skipped: 0,
            seq: Seq::from_slice(data?.as_slice())
        })
    }

    pub fn from_slice(s: &[Base]) -> Self {
        Dna { skipped: 0, seq: Seq::from_slice(s) }
    }

    pub fn prefix(&self, size: usize) -> Vec<Base> {
        self.to_vec(0..size)
    }

    pub fn skip(&mut self, count: usize) {
        self.skipped += count;
        if self.skipped > MAX_LEAF {
            self.seq = self.seq.subseq(self.skipped..);
            self.skipped = 0;
        }
    }

    pub fn to_vec(&self, range: std::ops::Range<usize>) -> Vec<Base> {
        self.seq.into_iter()
            .skip(self.skipped)
            .skip(range.start)
            .take(range.count())
            .cloned()
            .collect()
    }

    pub fn nth(&self, idx: usize) -> Option<Base> {
        self.seq.nth(idx + self.skipped).cloned()
    }

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn subseq(&self, range: std::ops::Range<usize>) -> Dna {
        let seq = self.seq.subseq(self.skipped..).subseq(range);
        Dna { seq, skipped: 0 }
    }

    pub fn concat(&self, other: &Self) -> Self {
        Dna { skipped: 0, seq: self.seq.subseq(self.skipped..).concat(&other.seq.subseq(other.skipped..)) }
    }
}

impl <'a> IntoIterator for &'a Dna {
    type Item = &'a Base;
    type IntoIter = Skip<SeqIter<'a, Base>>;

    fn into_iter(self) -> Self::IntoIter {
        self.seq.into_iter().skip(self.skipped)
    }
}

