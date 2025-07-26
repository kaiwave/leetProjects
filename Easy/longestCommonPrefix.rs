// yk what we're doing (prob 14)

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() { return String::new(); }

    let strs_chars: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
    let first_word = &strs_chars[0];
    let mut ans = String::new();

    for i in 0..first_word.len() {
      let letter = first_word[i];
      for word in &strs_chars {
        if i >= word.len() || word[i] != letter {
          return ans;
        }
      }
      ans.push(letter);
    }

    ans
  }
}