// Problem 14

class Solution {
public:
  string longestCommonPrefix(vector<string>& strs) {
    int strsLength = strs.size();
    int wordLength = 0;
    string firstWord = strs[0];
    char letter = firstWord[0];
    string ans = "";
    
    for (int i = 0; i < firstWord.length(); i++){
      letter = firstWord[i];
      for (int j = 0; j < strsLength; j++){
        if (letter == strs[j][i] && j == strsLength - 1){
          ans.push_back(letter);
        } else if (letter != strs[j][i]) {
          return ans;
        }
      }
    }
    return ans;
  }
};
