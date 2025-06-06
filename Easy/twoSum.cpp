// Problem 1!!

class Solution {
public:
  vector<int> twoSum(vector<int>& nums, int target) {
    vector<int> indx(2);
    int n = nums.size();
    for (int i = 0; i < n; i++) {
      for (int j = i + 1; j < n; j++) {
        if (nums[i] + nums[j] == target) {
          indx[0] = i;
          indx[1] = j;
          return indx;
        }
      }
    }
    return {};
  }
};
