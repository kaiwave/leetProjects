// 5 x 25 (125 characters) <- what the auto fill)

impl Solution {
  pub fn is_palindrome(s: String) -> bool {
    let cleaned: Vec<char> = s
    .chars()
    .filter(|c| c.is_ascii_alphanumeric())
    .flat_map(|c| c.to_lowercase())
    .collect();

    if cleaned.len() < 2 {
      return true;
    }
    
    let mut left = 0;
    let mut right = cleaned.len() - 1;

    while left < right {
      if cleaned[left] != cleaned[right] {
        return false;
      }
      left += 1;
      right -= 1;
    }

    true
  }
}