// problem 13

impl Solution {
  pub fn roman_to_int(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
      let mut i = 0;
      let mut num = 0;

      while i < chars.len() {
        let current = chars[i];
        let next = if i + 1 < chars.len() { Some(chars[i + 1]) } else { None };

        match (current, next) {
          ('I', Some('V')) => { num += 4; i += 2; }
          ('I', Some('X')) => { num += 9; i += 2; }
          ('X', Some('L')) => { num += 40; i += 2; }
          ('X', Some('C')) => { num += 90; i += 2; }
          ('C', Some('D')) => { num += 400; i += 2; }
          ('C', Some('M')) => { num += 900; i += 2; }

          _ => {
            num += match current {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
            };
          i += 1;
        }
      }
    }
    num
  }
}