# Advent of Code 2018
These are my solutions to the problems for Advent of Code 2018.

https://adventofcode.com/2018

## Day 1
### Part One
This problem is just a number summing problem. Each of the frequency changes is an integer and the solution is achieved by simply summing them all up.

Since all of the numbers in the input are reasonably small and there are not a huge number of them, integer overflows and underflows are unlikely.

```
$ cargo run -- day1 one < inputs/1.txt
411
```

### Part Two
This problem extends on the first part by having you look for repeats in the sum as each number is added. This can be done by putting each partial sum in a HashMap and doing lookups each time you add a number to the sum to see if you have already found this partial sum.

Since the partial sum repeats may occur only on multiple iterations through the list of changes, you need to continually re-loop through the list of changes until you hit repeat a partial sum. An effect of this is that if the partial sums do not repeat, then the program will not halt.

```
$ cargo run -- day1 two < inputs/1.txt
56360
```
