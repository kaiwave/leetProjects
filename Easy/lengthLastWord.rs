// problem 58

impl Solution {
  pub fn length_of_last_word(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut i = bytes.len() as i32 - 1;
    let mut length = 0;

    while i >= 0 && bytes[i as usize] == b' ' {
      i -= 1;
    }

    while i >= 0 && bytes[i as usize] != b' ' {
      length += 1;
      i -= 1;
    }

    length
  }
}