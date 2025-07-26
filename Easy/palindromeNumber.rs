// problem 9 but rusty

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    let mut reverse: i32 = 0;
    let mut temp: i32 = x;
    while temp != 0{
      reverse = (reverse * 10) + (temp % 10);
      temp = temp / 10;
    }

    if reverse == x && x >= 0{
      true
    } else {
      false
    }
  }
}