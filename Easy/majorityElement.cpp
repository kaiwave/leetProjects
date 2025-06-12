// Problem 169

class Solution {
public:
  int majorityElement(vector<int>& nums) {
    int majority = 0, count = 0;
    
    for (int element : nums) {
      if (count == 0) {
        majority = element;
      }

      if (element == majority) {
        count++;
      } else {
        count--;
      }
    }
    
    return majority;
  }
};
