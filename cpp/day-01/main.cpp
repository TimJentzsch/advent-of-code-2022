#include <iostream>
#include <fstream>
#include <string>

int main(int argc, char const *argv[])
{
  std::ifstream inputFile;
  inputFile.open("input.txt");

  int maxCalories = 0;
  int curCalories = 0;

  std::string line;

  while (!inputFile.eof())
  {
    std::getline(inputFile, line);

    // An empty line means that the next inventory starts
    if (line.length() == 0)
    {
      maxCalories = std::max(maxCalories, curCalories);
      curCalories = 0;
    }
    else
    {
      int calories = std::stoi(line);
      curCalories += calories;
    }
  }

  inputFile.close();

  // Print solution
  std::cout << maxCalories << "\n";

  return 0;
}
