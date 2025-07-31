// problem 35

impl Solution {
  pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
      let mid = (left + right) / 2;
      if nums[mid as usize] < target {
        left = mid + 1;
      } else if nums[mid as usize] > target {
        right = mid - 1;
      } else {
        return mid;
      }
    }
    left
  }
}