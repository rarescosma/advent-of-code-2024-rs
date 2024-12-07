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
