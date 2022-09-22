use xi_rope::{Interval};
use xi_rope::tree::{Leaf, Node, NodeInfo, TreeBuilder};
use std::marker::PhantomData;
use std::cmp::{min};
use xi_rope::interval::IntervalBounds;

pub const MAX_LEAF: usize = 1024;
pub const MIN_LEAF: usize = MAX_LEAF / 2 - 1;

#[derive(Clone)]
pub struct SeqLeaf<T> where T: Clone, T: Default {
    used: usize,
    data: [T; MAX_LEAF]
}

impl <T> SeqLeaf<T> where T: Clone + Copy + Default {
    pub fn from_slice(source: &[T]) -> Self where T: Default {
        let mut leaf = SeqLeaf::default();
        leaf.extend(source);
        leaf
    }

    pub fn extend(&mut self, source: &[T]) {
        let source_len = source.len();
        self.data[self.used..(self.used + source_len)].clone_from_slice(source);
        self.used += source_len;
    }

    pub fn shrink(&mut self, new_size: usize) -> &[T] {
        let rest = &self.data[new_size..self.used];
        self.used = new_size;
        return rest
    }
}

impl <T> Default for SeqLeaf<T> where T: Clone + Copy + Default {
    fn default() -> Self {

        SeqLeaf {
            used: 0,
            data: [T::default(); MAX_LEAF]
        }
    }
}

impl <T> Leaf for SeqLeaf<T> where T: Clone +Copy + Default {
    fn len(&self) -> usize {
        self.used
    }

    fn is_ok_child(&self) -> bool {
        self.used >= MIN_LEAF
    }

    fn push_maybe_split(&mut self, other: &Self, iv: Interval) -> Option<Self>  {
        let (start, end) = iv.start_end();
        if self.used + iv.size() <= MAX_LEAF {
            self.extend(&other.data[start..end]);
            None
        } else {
            let new_left_leaf_size = min(MAX_LEAF, self.used + iv.size() - MIN_LEAF);
            if new_left_leaf_size > self.used {
                let splitpoint = new_left_leaf_size - self.used;
                self.extend(&other.data[start..(start + splitpoint)]);
                let right_leaf = SeqLeaf::from_slice(&other.data[(start + splitpoint)..end]);
                Some(right_leaf)
            } else {
                let mut right_leaf = SeqLeaf::from_slice(self.shrink(new_left_leaf_size));
                right_leaf.extend(&other.data[start..end]);
                Some(right_leaf)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct SeqInfo<'a, T> {
    phantom: PhantomData<&'a T>
}

impl <T> NodeInfo for SeqInfo<'_, T> where T: Clone + Copy + Default {
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
pub struct Seq<T>(Node<SeqInfo<'static, T>>) where T: Clone + Copy + Default + 'static;

pub struct SeqIter<'a, T> where T: Clone + Copy + Default + 'static {
    offset: usize,
    cursor: xi_rope::Cursor<'a, SeqInfo<'static, T>>,
}

impl<'a, T> IntoIterator for &'a Seq<T> where T: Clone + Copy + Default {
    type Item = &'a T;
    type IntoIter = SeqIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SeqIter {
            offset: 0,
            cursor: xi_rope::Cursor::new(&self.0, 0),
        }
    }
}

impl<'a, T> Iterator for SeqIter<'a, T> where T: Clone + Copy + Default {
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

impl <T> Seq<T> where T: Clone + Copy + Default {
    pub fn from_slice(mut v: &[T]) -> Seq<T> {
        let mut b: TreeBuilder<SeqInfo<'static, T>> = TreeBuilder::new();
        if v.len() <= MAX_LEAF {
            if !v.is_empty() {
                b.push_leaf(SeqLeaf::from_slice(&v));
            }
            return Seq(b.build());
        }
        while !v.is_empty() {
            let splitpoint = if v.len() > MAX_LEAF {
                min(MAX_LEAF, v.len() - MIN_LEAF)
            } else {
                v.len()
            };
            b.push_leaf(SeqLeaf::from_slice(&v[..splitpoint]));
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