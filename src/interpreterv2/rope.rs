use xi_rope::{Interval};
use xi_rope::tree::{Leaf, Node, NodeInfo, TreeBuilder};
use std::marker::PhantomData;
use std::cmp::{min};
use xi_rope::interval::IntervalBounds;

#[derive(Clone)]
pub struct SeqLeaf<T> where T: Clone {
    data: Vec<T>
}

impl <T> Default for SeqLeaf<T> where T: Clone {
    fn default() -> Self {
        SeqLeaf {
            data: Vec::new()
        }
    }
}

const MIN_LEAF: usize = 511;
const MAX_LEAF: usize = 1024;

impl <T> Leaf for SeqLeaf<T> where T: Clone {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_ok_child(&self) -> bool {
        self.data.len() >= MIN_LEAF
    }

    fn push_maybe_split(&mut self, other: &Self, iv: Interval) -> Option<Self> {
        let (start, end) = iv.start_end();
        self.data.extend(other.data[start..end].iter().cloned());
        if self.len() <= MAX_LEAF {
            None
        } else {
            let splitpoint = min(MAX_LEAF, self.data.len() - MIN_LEAF);
            let right_leaf = SeqLeaf { data: self.data[splitpoint..].to_vec() };
            self.data.truncate(splitpoint);
            self.data.shrink_to_fit();
            Some(right_leaf)
        }
    }
}

#[derive(Clone, Copy)]
pub struct SeqInfo<'a, T> {
    // TODO get rid of size?
    // size: usize,
    phantom: PhantomData<&'a T>
}

impl <T> NodeInfo for SeqInfo<'_, T> where T: Clone {
    type L = SeqLeaf<T>;

    fn accumulate(&mut self, _other: &Self) {
    }

    fn compute_info(_leaf: &Self::L) -> Self {
        SeqInfo {
            phantom: Default::default()
        }
    }

    fn identity() -> Self {
        SeqInfo {
            phantom: Default::default()
        }
    }
}

#[derive(Clone)]
pub struct Seq<T>(Node<SeqInfo<'static, T>>) where T: Clone + 'static;

pub struct SeqIter<'a, T> where T: Clone + 'static {
    offset: usize,
    cursor: xi_rope::Cursor<'a, SeqInfo<'static, T>>,
}

impl<'a, T> IntoIterator for &'a Seq<T> where T: Clone {
    type Item = &'a T;
    type IntoIter = SeqIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SeqIter {
            offset: 0,
            cursor: xi_rope::Cursor::new(&self.0, 0),
        }
    }
}

impl<'a, T> Iterator for SeqIter<'a, T> where T: Clone {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.set(self.offset);
        self.offset += 1;
        let (leaf, offset) = self.cursor.get_leaf()?;
        if offset < leaf.len() {
            Some(&leaf.data[offset])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.cursor.total_len() - self.offset;
        (remaining, Some(remaining))
    }
}

impl <T> Seq<T> where T: Clone {
    pub fn from_slice(mut v: &[T]) -> Seq<T> {
        let mut b: TreeBuilder<SeqInfo<'static, T>> = TreeBuilder::new();
        if v.len() <= MAX_LEAF {
            if !v.is_empty() {
                b.push_leaf(SeqLeaf { data: v.to_vec() });
            }
            return Seq(b.build());
        }
        while !v.is_empty() {
            let splitpoint = if v.len() > MAX_LEAF {
                min(MAX_LEAF, v.len() - MIN_LEAF)
            } else {
                v.len()
            };
            b.push_leaf(SeqLeaf { data: v[..splitpoint].to_vec() });
            v = &v[splitpoint..];
        }
        Seq(b.build())
    }

    pub fn nth(&self, offset: usize) -> Option<&T> {
        let cursor = xi_rope::Cursor::new(&self.0, offset);
        let (leaf, leaf_offset) = cursor.get_leaf()?;
        if leaf_offset < leaf.len() {
            Some(&leaf.data[leaf_offset])
        } else {
            None
        }
    }

    pub fn subseq<I: IntervalBounds>(&self, iv: I) -> Self {
        Seq(self.0.subseq(iv))
    }

    pub fn concat(&self, other: &Self) -> Self {
        let mut builder = TreeBuilder::new();
        builder.push(self.0.clone());
        builder.push(other.0.clone());
        Seq(builder.build())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn base_test() {
        let s = Seq::from_slice(&[1, 2, 3]);
        assert_eq!(s.0.len(), 3);
        assert_eq!(s.nth(1), Some(&2));
        assert_eq!(s.nth(3), None);
    }

    #[test]
    fn big_test() {
        let data: Vec<_> = (0..10_000_000).collect();
        let before_build = Instant::now();
        let s = Seq::from_slice(data.as_slice());
        eprintln!("Seq building took: {:?}", before_build.elapsed());
        for (i, x) in data.iter().enumerate() {
            assert_eq!(s.nth(i), Some(x));
        }
    }

    #[test]
    fn split_test() {
        let data: Vec<_> = (0..10_000_000).collect();
        let s = Seq::from_slice(data.as_slice());
        let s = s.subseq(5_000_000..);
        assert_eq!(s.nth(0), Some(&5_000_000));
    }

    #[test]
    fn iter_test() {
        let data: Vec<_> = (0..10_000_000).collect();
        let s = Seq::from_slice(data.as_slice());
        assert_eq!(s.into_iter().cloned().collect::<Vec<_>>(), data);
    }

    #[test]
    fn size_hint_test() {
        let data: Vec<_> = (0..2000).collect();
        let s = Seq::from_slice(data.as_slice());
        let mut iter = s.into_iter();
        assert_eq!(iter.size_hint(), (2000, Some(2000)));
        iter.next();
        assert_eq!(iter.size_hint(), (1999, Some(1999)));
    }
}