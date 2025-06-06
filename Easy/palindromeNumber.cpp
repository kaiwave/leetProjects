// Problem 9 :p

class Solution {
public:
  bool isPalindrome(int x) {
    long long reverse = 0;
    long long temp = x;
    while (temp != 0){
      reverse = (reverse * 10) + (temp % 10);
      temp = temp / 10;
    }
    
    if (reverse == x && x >= 0){
      return true;
    } else {
      return false;
    }
  }
};
