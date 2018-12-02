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

## Day 2
### Part One
This problem is a variation on a frequency table question. For each input string we need to count up the number of each character used and find if any characters are used exactly 2 or 3 times.

The character counting can be done by using a HashMap as a frequency table. We simply go over each character in the string, look it up in the HashMap, if we have not seen it yet, then set its value by 1, if we have seen it before then increment its count by 1.

Once we count up the number of strings with cases of characters that have been repeated 2 or 3 times, then we can simply multiply those two counts together to get the answer.

```
$ cargo run -- day2 one < inputs/2.txt
5904
```

### Part Two
This problem appears at first to have a simple solution in just comparing each input string against each other string to find the pair that differs only by one character. However, this results in an algorithm that is likely to be too inefficient to run in a reasonable amount of time.

To make a solution that is more efficent we can try to come up with a heuristic to reduce the number of compairisons that we need to do. One possible approach would be to somehow hash the input strings so that strings with only 1 character differences would have the same hash, and most strings differing by more than 1 character would have different hashes.

However, the required 1 character difference makes it tricky to come up with a working has function, since requardless of which 1 character is different, the hash should still turn out the same. I ended up getting around this by simply producing separate hashes for each possible version of the string with 1 character removed. Then the hashes and associated strings can be put into a MultiMap, and as we hash new input strings we can use the MultiMap to check for previous instances of the hash and only check those for 1 character differences.

In terms of the hash function to use on the 1 character removed strings, we can simply take the ascii values of the characters and sum them together. If this hash function led to too many collisions we could also change the addition to multiplication, map the ascii character values to primes, use larger numerical types, etc.. However, the simple summing hash function seemd to work well enough.

Then when we find the matching pair, we can simply filter them down to the characters in common by iterating over both of them simultaneously and filtering out the non-matching characters.

```
$ cargo run -- day2 two < inputs/2.txt
jiwamotgsfrudclzbyzkhlrvp
```
