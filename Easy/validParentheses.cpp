// Problem 20

class Solution {
public:
  bool isValid(string s) {
    stack<char> brackets;

    for (int i = 0; i < s.length(); i++) {
      char c = s[i];
      if (c == '(' || c == '[' || c == '{') {
        brackets.push(c);
      }

      else {
        if (brackets.empty()) return false;
        char top = brackets.top();
        if ((c == ')' && top == '(') || (c == ']' && top == '[') || (c == '}' && top == '{')) {
          brackets.pop();
        }
        else {
          return false;
        }
      }
    }

    return brackets.empty();
  }
};
