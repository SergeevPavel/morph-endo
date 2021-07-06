use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Base {
    I, C, F, P
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dna {
    pub data: Vec<Base>
}

impl Dna {
    pub fn from_string(s: &str) -> Result<Dna, String> {
        let data: Result<Vec<Base>, String> = s.chars().map(|c| {
            match c {
                'I' => Ok(Base::I),
                'C' => Ok(Base::C),
                'F' => Ok(Base::F),
                'P' => Ok(Base::P),
                other => Err(format!("Unexpected symbol {}", other).to_string())
            }
        }).collect();
        return data.map(|data| Dna { data })
    }

    pub fn empty() -> Dna {
        Dna { data: vec![] }
    }

    pub fn prep(&mut self, b: Base) {
        self.data.insert(0, b)
    }

    pub fn app(&mut self, b: Base) {
        self.data.push(b)
    }

    pub fn concat(&mut self, other: &Dna) {
        self.data.extend_from_slice(&other.data)
    }

    pub fn nth(&self, n: usize) -> Option<Base> {
        self.data.get(n).map(|b| *b)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub trait Subseq<Interval> {
    fn subseq(&self, interval: Interval) -> Dna;
}

impl Subseq<std::ops::Range<usize>> for Dna {
    fn subseq(&self, interval: std::ops::Range<usize>) -> Dna {
        let from = interval.start.max(0).min(self.data.len());
        let to = interval.end.max(0).min(self.data.len());
        if from < to {
            Dna { data: self.data[from..to].to_vec() }
        } else {
            Dna::empty()
        }
    }
}

impl Subseq<std::ops::RangeFrom<usize>> for Dna {
    fn subseq(&self, interval: std::ops::RangeFrom<usize>) -> Dna {
        let from = interval.start.max(0);
        if from < self.data.len() {
            Dna { data: self.data[from..].to_vec() }
        } else {
            Dna::empty()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dna(s: &str) -> Dna {
        Dna::from_string(s).unwrap()
    }
    
    #[test]
    fn subseq() {
        let icfp = dna("ICFP");
        assert_eq!(icfp.subseq(0..2), dna("IC"));
        assert_eq!(icfp.subseq(2..0), Dna::empty());
        assert_eq!(icfp.subseq(2..2), Dna::empty());
        assert_eq!(icfp.subseq(2..3), dna("F"));
        assert_eq!(icfp.nth(2), Some(Base::F));
        assert_eq!(icfp.subseq(2..6), dna("FP"));
        assert_eq!(icfp.subseq(2..), dna("FP"));
        assert_eq!(icfp.nth(6), None);
        assert_eq!(icfp.subseq(3..), dna("P"));
        assert_eq!(icfp.subseq(4..), Dna::empty());
    }
}