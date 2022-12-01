#pragma once

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>

std::ifstream getInputFile(std::string identifier)
{
  std::string fileName = "day_" + identifier + ".txt";

  std::ifstream inputFile;
  inputFile.open("../inputs/" + fileName);

  if (!inputFile)
  {
    throw std::runtime_error("The input file doesn't exist, make sure you create it at /inputs/" + fileName);
  }

  return inputFile;
}
