// 1480

impl Solution {
  pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let vector_length = nums.len();
    let mut result = vec![0; vector_length];

    if vector_length == 0 {
      return result;
    }

    for i in 0..vector_length {
      if i == 0 {
        result[i] = nums[i];
      } else {
        result[i] = result[i - 1] + nums[i];
      }
    }

    result
  }
}