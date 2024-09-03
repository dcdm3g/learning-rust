#[allow(dead_code)]
fn execute(hours: f64) -> i32 {
  (hours * 0.5) as i32
}

#[cfg(test)]
mod tests {
  use super::execute;

  #[test]
  fn sample_tests() {
      assert_eq!(execute(2.), 1);
      assert_eq!(execute(1.4), 0);
      assert_eq!(execute(12.3), 6);
      assert_eq!(execute(0.82), 0);
      assert_eq!(execute(11.8), 5);
      assert_eq!(execute(1787.), 893);
      assert_eq!(execute(0.), 0);
  }
}