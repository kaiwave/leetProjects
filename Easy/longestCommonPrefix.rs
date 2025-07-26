// yk what we're doing (prob 14)

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
      return String::new();
    }

    let strs_length = strs.len();
    let first_word: Vec<char> = strs[0].chars().collect();
    let mut ans = String::new();

    for i in 0..first_word.len() {
      let letter = first_word[i];

      for j in 0..strs_length {
        let current_word: Vec<char> = strs[j].chars().collect();

        if i >= current_word.len() || current_word[i] != letter {
          return ans;
        }
      }

      ans.push(letter);
    }

    ans
  }
}