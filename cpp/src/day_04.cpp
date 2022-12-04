#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <cassert>
#include "utils.hpp"

const std::string IDENTIFIER = "04";

using Range = std::array<size_t, 2>;

Range parseRange(std::string input)
{
  auto pos = input.find_first_of('-');

  auto startStr = input.substr(0, pos);
  auto endStr = input.substr(pos + 1);

  auto start = (size_t)std::stoi(startStr);
  auto end = (size_t)std::stoi(endStr);

  return Range{start, end};
}

std::tuple<Range, Range> parseRanges(std::string input)
{
  auto pos = input.find_first_of(',');

  auto firstStr = input.substr(0, pos);
  auto secondStr = input.substr(pos + 1);

  auto firstRange = parseRange(firstStr);
  auto secondRange = parseRange(secondStr);

  return std::tuple{firstRange, secondRange};
}

bool rangeEquals(Range range1, Range range2)
{
  return range1.at(0) == range2.at(0) && range1.at(1) == range2.at(1);
}

bool rangeContains(Range range1, Range range2)
{
  return range1.at(0) <= range2.at(0) && range1.at(1) >= range2.at(1);
}

void tests()
{
  assert(rangeEquals(Range{1, 4}, Range{1, 4}));
  assert(!rangeEquals(Range{2, 6}, Range{3, 6}));
  assert(!rangeEquals(Range{2, 6}, Range{2, 7}));

  assert(rangeContains(Range{1, 4}, Range{1, 4}));
  assert(rangeContains(Range{1, 4}, Range{2, 4}));
  assert(rangeContains(Range{1, 4}, Range{1, 3}));
  assert(!rangeContains(Range{2, 6}, Range{1, 6}));
  assert(!rangeContains(Range{2, 6}, Range{2, 7}));

  assert(rangeEquals(parseRange("2-4"), Range{2, 4}));

  auto ranges = parseRanges("2-3,4-5");
  assert(rangeEquals(std::get<0>(ranges), Range{2, 3}));
  assert(rangeEquals(std::get<1>(ranges), Range{4, 5}));

  std::cout << "All tests passed!" << std::endl;
}

int main(int argc, char const *argv[])
{
  tests();

  std::ifstream inputFile = getInputFile(IDENTIFIER);

  std::string line;

  size_t part1Count = 0;

  while (std::getline(inputFile, line))
  {
    if (line.empty())
    {
      continue;
    }

    auto ranges = parseRanges(line);
    auto range1 = std::get<0>(ranges);
    auto range2 = std::get<1>(ranges);

    if (rangeContains(range1, range2) || rangeContains(range2, range1))
    {
      part1Count += 1;
    }
  }
  std::cout << "Part 1: " << part1Count << std::endl;
  std::cout << "Part 2: " << std::endl;

  return 0;
}
