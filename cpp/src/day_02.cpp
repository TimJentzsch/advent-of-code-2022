#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <cassert>
#include "utils.hpp"

const std::string IDENTIFIER = "02";

enum RpsShape
{
  ROCK = 1,
  PAPER = 2,
  SCISSORS = 3,
};

enum RpsOutcome
{
  LOSS = 0,
  DRAW = 3,
  WIN = 6,
};

RpsShape getEnemyShape(char input)
{
  switch (input)
  {
  case 'A':
    return RpsShape::ROCK;
  case 'B':
    return RpsShape::PAPER;
  case 'C':
    return RpsShape::SCISSORS;
  default:
    std::string inputStr(1, input);
    throw std::runtime_error("'" + inputStr + "' is an invalid option for the enemy.");
  }
}

RpsShape getMyShape(char input)
{
  switch (input)
  {
  case 'X':
    return RpsShape::ROCK;
  case 'Y':
    return RpsShape::PAPER;
  case 'Z':
    return RpsShape::SCISSORS;
  default:
    std::string inputStr(1, input);
    throw std::runtime_error("'" + inputStr + "' is an invalid option for you.");
  }
}

RpsOutcome getMyOutcome(RpsShape enemyShape, RpsShape myShape)
{
  if (enemyShape == myShape)
  {
    return RpsOutcome::DRAW;
  }

  // TODO: We could probably use modulo arithmetic to simplify this
  if (
      (enemyShape == RpsShape::PAPER && myShape == RpsShape::ROCK) || (enemyShape == RpsShape::ROCK && myShape == RpsShape::SCISSORS) || (enemyShape == RpsShape::SCISSORS && myShape == RpsShape::PAPER))
  {
    return RpsOutcome::LOSS;
  }

  return RpsOutcome::WIN;
}

// Who's gonna stop me?
void tests()
{
  assert(getEnemyShape('A') == RpsShape::ROCK);
  assert(getEnemyShape('B') == RpsShape::PAPER);
  assert(getEnemyShape('C') == RpsShape::SCISSORS);

  assert(getMyShape('X') == RpsShape::ROCK);
  assert(getMyShape('Y') == RpsShape::PAPER);
  assert(getMyShape('Z') == RpsShape::SCISSORS);

  assert(getMyOutcome(RpsShape::ROCK, RpsShape::ROCK) == RpsOutcome::DRAW);
  assert(getMyOutcome(RpsShape::ROCK, RpsShape::PAPER) == RpsOutcome::WIN);
  assert(getMyOutcome(RpsShape::ROCK, RpsShape::SCISSORS) == RpsOutcome::LOSS);
  assert(getMyOutcome(RpsShape::PAPER, RpsShape::ROCK) == RpsOutcome::LOSS);
  assert(getMyOutcome(RpsShape::PAPER, RpsShape::PAPER) == RpsOutcome::DRAW);
  assert(getMyOutcome(RpsShape::PAPER, RpsShape::SCISSORS) == RpsOutcome::WIN);
  assert(getMyOutcome(RpsShape::SCISSORS, RpsShape::ROCK) == RpsOutcome::WIN);
  assert(getMyOutcome(RpsShape::SCISSORS, RpsShape::PAPER) == RpsOutcome::LOSS);
  assert(getMyOutcome(RpsShape::SCISSORS, RpsShape::SCISSORS) == RpsOutcome::DRAW);
}

int main(int argc, char const *argv[])
{
  tests();

  std::ifstream inputFile = getInputFile(IDENTIFIER);

  std::string line;
  u_int total_score = 0;

  while (std::getline(inputFile, line))
  {
    if (!line.empty())
    {
      char enemyChoice = line.at(0);
      char myChoice = line.at(2);

      auto enemyShape = getEnemyShape(enemyChoice);
      auto myShape = getMyShape(myChoice);

      auto outcome = getMyOutcome(enemyShape, myShape);

      total_score += myShape + outcome;
    }
  }

  std::cout << "Part 1: " << total_score << std::endl;

  return 0;
}
