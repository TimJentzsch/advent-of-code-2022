CUR_DAY="04"

# Determine the day to execute
if [ $# -gt 0 ]
then
  day="$1"
else
  day="$CUR_DAY"
fi

# Create out directory if it doesn't exist
mkdir -p -- out

# Compile the program for the given day
g++ -o out/day_$day.out src/day_$day.cpp
# Execute the program
./out/day_$day.out
