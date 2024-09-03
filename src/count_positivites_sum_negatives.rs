#[allow(dead_code)]
pub fn execute(input: Vec<i32>) -> Vec<i32> {
  if input.is_empty() {
    return vec![];
  }

  input.iter().fold(vec![0, 0], | mut acc: Vec<i32>, cur: &i32 | {
    if *cur > 0 {
      acc[0] += 1;
    } else {
      acc[1] += cur;
    }

    acc
  })
}

#[cfg(test)]
mod tests {
    use super::execute;
        
    fn dotest(arr: &[i32], expected: &[i32]) {
        let actual: Vec<i32> = execute(arr.to_vec());
        assert!(actual == expected, 
            "With input = {arr:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn test_count_pisitives_sum_negatives() {
        dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
        dotest(&[], &[]);
        dotest(&[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14], &[8, -50]);
        dotest(&[0,1,2,3,4,5], &[5, 0]);
        dotest(&[1,2,3,4,5], &[5, 0]);
        dotest(&[0,-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[0,0,0,0], &[0,0]);
        dotest(&[0], &[0,0]);
    }
}
