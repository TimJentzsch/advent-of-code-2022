#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <bits/stdc++.h>

int main(int argc, char const *argv[])
{
  std::ifstream inputFile;
  inputFile.open("input.txt");

  std::vector<int> elfCalories;
  int curCalories = 0;

  std::string line;

  while (!inputFile.eof())
  {
    std::getline(inputFile, line);

    // An empty line means that the next inventory starts
    if (line.length() == 0)
    {
      elfCalories.push_back(curCalories);
      curCalories = 0;
    }
    else
    {
      int calories = std::stoi(line);
      curCalories += calories;
    }
  }

  inputFile.close();

  // Sort the calories in descending order
  std::sort(elfCalories.begin(), elfCalories.end(), std::greater<int>());

  // Part 1 solution
  std::cout << "Maximum calories: " << elfCalories[0] << "\n";

  // Part 2 solution
  int topThreeCalories = elfCalories[0] + elfCalories[1] + elfCalories[2];
  std::cout << "Top 3 calories: " << topThreeCalories << "\n";

  return 0;
}
