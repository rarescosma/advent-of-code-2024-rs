use std::{hash::Hash, str::FromStr};

use aoc_prelude::HashMap;

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
