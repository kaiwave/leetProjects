// Problem 28

class Solution {
public:
  int strStr(string haystack, string needle) {
    if (needle.length() > haystack.length()) {
      return -1;
    }
    else if (needle.length() == haystack.length()) {
      if (needle == haystack) {
        return 0;
      }
      else {
        return -1;
      }
    }

    string check = "";
    int ans = 0;

    for (int i = 0; i < haystack.length(); i++) {
      check.push_back(haystack[i]);
      for (int j = 0; j < needle.length(); j++) {
        if (check.length() == needle.length()) {
          if (check == needle){
            return ans;
          }
          else {
            check.erase(0, 1);
            ans = ans + 1;
          }
        } 
      }
    }
    if (check == needle) {
      return ans;
    }
    else {
      return -1;
    }
  }
};