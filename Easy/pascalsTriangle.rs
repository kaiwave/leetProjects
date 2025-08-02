// problem 118

impl Solution {
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if num_rows <= 0 {
      return result;     
  }
    for i in 0..num_rows {
      let mut row = vec![1; (i + 1) as usize];
      for j in 1..i {
        row[j as usize] = result[(i - 1) as usize][j as usize - 1] + result[(i - 1) as usize][j as usize];
      }
      result.push(row);
    }
    result
  }
}