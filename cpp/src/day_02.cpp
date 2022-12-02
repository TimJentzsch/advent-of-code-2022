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

RpsShape parseEnemyShape(char input)
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

RpsShape parseMyShape(char input)
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

RpsOutcome calculateMyOutcome(RpsShape enemyShape, RpsShape myShape)
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

RpsOutcome parseMyOutcome(char input)
{
  switch (input)
  {
  case 'X':
    return RpsOutcome::LOSS;
  case 'Y':
    return RpsOutcome::DRAW;
  case 'Z':
    return RpsOutcome::WIN;
  default:
    std::string inputStr(1, input);
    throw std::runtime_error("'" + inputStr + "' is an invalid outcome for you.");
  }
}

RpsShape calculateMyShape(RpsShape enemyShape, RpsOutcome myOutcome)
{
  if (myOutcome == RpsOutcome::DRAW)
  {
    return enemyShape;
  }

  if (myOutcome == RpsOutcome::LOSS)
  {
    switch (enemyShape)
    {
    case RpsShape::ROCK:
      return RpsShape::SCISSORS;
    case RpsShape::PAPER:
      return RpsShape::ROCK;
    case RpsShape::SCISSORS:
      return RpsShape::PAPER;
    }
  }

  switch (enemyShape)
  {
  case RpsShape::ROCK:
    return RpsShape::PAPER;
  case RpsShape::PAPER:
    return RpsShape::SCISSORS;
  case RpsShape::SCISSORS:
    return RpsShape::ROCK;
  }

  // C++ cannot deduct that this is unreachable
  throw std::runtime_error("Invalid combination");
}

// Who's gonna stop me?
void tests()
{
  assert(parseEnemyShape('A') == RpsShape::ROCK);
  assert(parseEnemyShape('B') == RpsShape::PAPER);
  assert(parseEnemyShape('C') == RpsShape::SCISSORS);

  assert(parseMyShape('X') == RpsShape::ROCK);
  assert(parseMyShape('Y') == RpsShape::PAPER);
  assert(parseMyShape('Z') == RpsShape::SCISSORS);

  assert(calculateMyOutcome(RpsShape::ROCK, RpsShape::ROCK) == RpsOutcome::DRAW);
  assert(calculateMyOutcome(RpsShape::ROCK, RpsShape::PAPER) == RpsOutcome::WIN);
  assert(calculateMyOutcome(RpsShape::ROCK, RpsShape::SCISSORS) == RpsOutcome::LOSS);
  assert(calculateMyOutcome(RpsShape::PAPER, RpsShape::ROCK) == RpsOutcome::LOSS);
  assert(calculateMyOutcome(RpsShape::PAPER, RpsShape::PAPER) == RpsOutcome::DRAW);
  assert(calculateMyOutcome(RpsShape::PAPER, RpsShape::SCISSORS) == RpsOutcome::WIN);
  assert(calculateMyOutcome(RpsShape::SCISSORS, RpsShape::ROCK) == RpsOutcome::WIN);
  assert(calculateMyOutcome(RpsShape::SCISSORS, RpsShape::PAPER) == RpsOutcome::LOSS);
  assert(calculateMyOutcome(RpsShape::SCISSORS, RpsShape::SCISSORS) == RpsOutcome::DRAW);

  assert(parseMyOutcome('X') == RpsOutcome::LOSS);
  assert(parseMyOutcome('Y') == RpsOutcome::DRAW);
  assert(parseMyOutcome('Z') == RpsOutcome::WIN);
}

int main(int argc, char const *argv[])
{
  tests();

  std::ifstream inputFile = getInputFile(IDENTIFIER);

  std::string line;
  u_int total_score_part_1 = 0;
  u_int total_score_part_2 = 0;

  while (std::getline(inputFile, line))
  {
    if (!line.empty())
    {
      char enemyChoice = line.at(0);
      char myChoice = line.at(2);

      auto enemyShape = parseEnemyShape(enemyChoice);

      // Part 1
      auto myParsedShape = parseMyShape(myChoice);
      auto myCalculatedOutcome = calculateMyOutcome(enemyShape, myParsedShape);
      total_score_part_1 += myParsedShape + myCalculatedOutcome;

      // Part 2
      auto myParsedOutcome = parseMyOutcome(myChoice);
      auto myCalculatedShape = calculateMyShape(enemyShape, myParsedOutcome);
      total_score_part_2 += myCalculatedShape + myParsedOutcome;
    }
  }

  std::cout << "Part 1: " << total_score_part_1 << std::endl;
  std::cout << "Part 2: " << total_score_part_2 << std::endl;

  return 0;
}
