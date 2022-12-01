#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include "utils.hpp"

#define IDENTIFIER "01"

int main(int argc, char const *argv[])
{
  std::ifstream inputFile = getInputFile(IDENTIFIER);

  std::vector<int> elfCalories;
  int curCalories = 0;

  std::string line;

  while (std::getline(inputFile, line))
  {
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

  // Sort the calories in descending order
  std::sort(elfCalories.begin(), elfCalories.end(), std::greater<>());

  // Part 1 solution
  std::cout << "Part 1: " << elfCalories.at(0) << std::endl;

  // Part 2 solution
  int topThreeCalories = elfCalories.at(0) + elfCalories.at(1) + elfCalories.at(2);
  std::cout << "Part 2: " << topThreeCalories << std::endl;

  return 0;
}
