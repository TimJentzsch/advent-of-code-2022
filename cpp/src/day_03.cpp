#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <cassert>
#include "utils.hpp"

const std::string IDENTIFIER = "03";

using Item = size_t;
using Prio = size_t;
constexpr Item ITEM_COUNT = 26 * 2;
using Items = std::array<Item, ITEM_COUNT>;

/**
 * The item index is the priority - 1.
 * `a` through `z` have intem indexes 0 through 25.
 */
Item inputToItemIndex(char input)
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

Items countCompartmentItems(std::string compartment)
{
  // Apparently this will initialize everything to 0 (hopefully)
  Items items = {0};

  for (auto &item : compartment)
  {
    items[item] += 1;
  }

  return items;
}

Prio duplicateItemPriority(Items firstItemCounts, Items secondItemCounts)
{
  for (Item i = 0; i < firstItemCounts.size(); i++)
  {
    if (firstItemCounts.at(i) > 0 && secondItemCounts.at(i) > 0)
    {
      auto priority = i + 1;
      return priority;
    }
  }

  return 0;
}

Prio tripleItemPriority(Items firstItemCounts, Items secondItemCounts, Items thirdItemCounts)
{
  for (Item i = 0; i < firstItemCounts.size(); i++)
  {
    if (firstItemCounts.at(i) > 0 && secondItemCounts.at(i) > 0 && thirdItemCounts.at(i) > 0)
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
  Prio part1Sum = 0;
  // Sum of priorities of key items
  Prio part2Sum = 0;

  // It's ugly but it works
  // Will count how many rucksacks we have "collected"
  size_t rucksackIndex = 0;
  std::array<Items, 3> rucksacks;

  while (std::getline(inputFile, line))
  {
    if (line.empty())
    {
      continue;
    }

    // PART 1

    auto length = line.size();
    // Devide the rucksack into the two compartments
    auto firstCompartment = line.substr(0, length / 2);
    auto secondCompartment = line.substr(length / 2);

    // Count how many of each item we have in each compartment
    auto firstItemCounts = countCompartmentItems(firstCompartment);
    auto secondItemCounts = countCompartmentItems(secondCompartment);

    // Determine duplicate items
    part1Sum += duplicateItemPriority(firstItemCounts, secondItemCounts);

    // PART 2

    rucksacks[rucksackIndex] = countCompartmentItems(line);

    if (rucksackIndex >= 2)
    {
      part2Sum += tripleItemPriority(rucksacks.at(0), rucksacks.at(1), rucksacks.at(2));
      rucksackIndex = 0;
    }
    else
    {
      rucksackIndex += 1;
    }
  }

  std::cout << "Part 1: " << part1Sum << std::endl;
  std::cout << "Part 2: " << part2Sum << std::endl;
}
