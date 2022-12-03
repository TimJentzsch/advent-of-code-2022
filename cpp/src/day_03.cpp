#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <cassert>
#include "utils.hpp"

const std::string IDENTIFIER = "03";

constexpr size_t ITEM_COUNT = 26 * 2;

/**
 * The item index is the priority - 1.
 * `a` through `z` have intem indexes 0 through 25.
 */
size_t inputToItemIndex(char input)
{
  if (input >= 'a' && input <= 'z')
  {
    return input - 'a';
  }

  if (input >= 'A' && input <= 'Z')
  {
    return input - 'A' + 26;
  }

  throw std::runtime_error("Invalid item input");
}

std::array<size_t, ITEM_COUNT> countCompartmentItems(std::string compartment)
{
  // Apparently this will initialize everything to 0 (hopefully)
  std::array<size_t, ITEM_COUNT> items = {0};

  for (size_t i = 0; i < compartment.size(); i++)
  {
    size_t index = inputToItemIndex(compartment.at(i));

    items[index] += 1;
  }

  return items;
}

size_t duplicateItemPriority(std::array<size_t, ITEM_COUNT> firstItemCounts, std::array<size_t, ITEM_COUNT> secondItemCounts)
{
  for (size_t i = 0; i < firstItemCounts.size(); i++)
  {
    if (firstItemCounts.at(i) > 0 && secondItemCounts.at(i) > 0)
    {
      auto priority = i + 1;
      return priority;
    }
  }

  return 0;
}

void test()
{
  assert(inputToItemIndex('a') == 0);
  assert(inputToItemIndex('z') == 25);
  assert(inputToItemIndex('A') == 26);
  assert(inputToItemIndex('Z') == 51);

  assert(duplicateItemPriority(countCompartmentItems("vJrwpWtwJgWr"), countCompartmentItems("hcsFMMfFFhFp")) == 16);
  assert(duplicateItemPriority(countCompartmentItems("jqHRNqRjqzjGDLGL"), countCompartmentItems("rsFMfFZSrLrFZsSL")) == 38);
  assert(duplicateItemPriority(countCompartmentItems("PmmdzqPrV"), countCompartmentItems("vPwwTWBwg")) == 42);
}

int main(int argc, char const *argv[])
{
  test();

  std::ifstream inputFile = getInputFile(IDENTIFIER);

  std::string line;

  // Sum of the priorities of duplicate items
  size_t part1Sum = 0;

  while (std::getline(inputFile, line))
  {
    auto length = line.size();
    // Devide the rucksack into the two compartments
    auto firstCompartment = line.substr(0, length / 2);
    auto secondCompartment = line.substr(length / 2);

    // Count how many of each item we have in each compartment
    auto firstItemCounts = countCompartmentItems(firstCompartment);
    auto secondItemCounts = countCompartmentItems(secondCompartment);

    // Determine duplicate items
    part1Sum += duplicateItemPriority(firstItemCounts, secondItemCounts);
  }

  std::cout << "Part 1: " << part1Sum << std::endl;
}
