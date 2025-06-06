class Solution {
public:
  int romanToInt(string s) {
    int I = 1, IV = 4, V = 5, IX = 9, X = 10, XL = 40, L = 50, XC = 90, C = 100, CD = 400, D = 500, CM = 900, M = 1000;
    int num = 0;

    for (int i = 0; i < s.length(); i++) {
      string z1;
      z1.push_back(s[i]);

      string z2 = "";
      if (i + 1 < s.length()) {
        z2.push_back(s[i + 1]);
      }

      string combined = z1 + z2;

      if (combined == "IV" || combined == "IX" || combined == "XL" || combined == "XC" || combined == "CD" || combined == "CM") {
        if (combined == "IV") num += IV;
        else if (combined == "IX") num += IX;
        else if (combined == "XL") num += XL;
        else if (combined == "XC") num += XC;
        else if (combined == "CD") num += CD;
        else if (combined == "CM") num += CM;
        i++;
      }
      else {
        if (z1 == "I") num += I;
        else if (z1 == "V") num += V;
        else if (z1 == "X") num += X;
        else if (z1 == "L") num += L;
        else if (z1 == "C") num += C;
        else if (z1 == "D") num += D;
        else if (z1 == "M") num += M;
      }
    }
    return num;
  }
};