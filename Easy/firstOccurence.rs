// proble 28

impl Solution {
  pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
      return 0;
    }

    if needle.len() > haystack.len() {
      return -1;
    }
    
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    
    for i in 0..=haystack.len() - needle.len() {
      if &haystack_bytes[i..i + needle.len()] == needle_bytes {
        return i as i32;
      }
    }
    
    -1      
  }
}