CUR_DAY="01"

if [ $# -gt 0 ]
then
  day="$1"
else
  day="$CUR_DAY"
fi

cd "day-$day"
g++ -o main.out main.cpp
./main.out
