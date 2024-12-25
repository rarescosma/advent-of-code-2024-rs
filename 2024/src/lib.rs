use std::{hash::Hash, ops::BitOr, str::FromStr};

use aoc_prelude::{num_integer::Integer, HashMap};

/// Macro for solution timing
/// Credits: <https://github.com/AxlLind>/
#[macro_export]
macro_rules! main {
  ($($body:tt)+) => {
    fn main() {
      let now = std::time::Instant::now();
      let (p1,p2) = { $($body)+ };
      let elapsed = now.elapsed();
      println!("Part one: {}", p1);
      println!("Part two: {}", p2);
      if elapsed.as_millis() > 0 {
        println!("Time: {}ms", elapsed.as_millis());
      } else {
        println!("Time: {}μs", elapsed.as_micros());
      }
    }
  }
}

#[inline]
pub fn extract_nums<'a, T: FromStr + 'a>(s: &'a str) -> impl Iterator<Item = T> + 'a {
    s.split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty()).flat_map(str::parse::<T>)
}

pub fn reverse<K, V: Eq + Hash>(h: HashMap<K, V>) -> HashMap<V, K> {
    h.into_iter().map(|(k, v)| (v, k)).collect()
}

#[derive(Copy, Clone, Debug)]
pub struct BitSet<const N: usize> {
    inner: [u128; N],
}

impl<const N: usize> Default for BitSet<N> {
    fn default() -> Self { Self { inner: [0u128; N] } }
}

impl<const N: usize> BitSet<N> {
    pub fn contains(&self, index: usize) -> bool {
        let (shard, shift) = index.div_rem(&128);
        let word = self.inner[shard];
        (word >> shift) & 1 == 1
    }

    pub fn set(&mut self, index: usize) {
        let (shard, shift) = index.div_rem(&128);
        let word = &mut self.inner[shard];
        *word |= 1 << shift
    }
}

impl<const N: usize> BitOr<usize> for BitSet<N> {
    type Output = Self;

    fn bitor(mut self, rhs: usize) -> Self::Output {
        self.set(rhs);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::BitSet;

    #[test]
    fn test_bitset() {
        let mut sut = BitSet::<4>::default();

        sut.set(3);
        assert!(!sut.contains(0));
        assert!(sut.contains(3));

        sut.set(152);
        assert!(!sut.contains(313));
        assert!(sut.contains(152));
    }
}
