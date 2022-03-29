# brute1000

## About

This program solves the game *Potz1000* by precomputing the best move regarding expected final score for every game state.
The underlying algorithm uses a dynamic programming sheme, because the naive brute force didn't finish in reasonable time on my laptop.

## Potz1000

* The game board of Potz1000 consists of 9 squares arranged in a 3x3 grid. 
* The squares are initially empty.
* In each turn of the game, the player rolls a 6-sided dice and places the resulting number in one of the squares.
* After 9 turns, all squares are filled and the final score is calculated.
* For calculating the score, the numbers in the left column are multiplied by 100, 
  the numbers in the middle column are multiplied by 10 and the numbers in the right column are multiplied by 1.
  The resulting numbers are then summed up. The absolute value of the difference between this sum and 1000 is the base score.
* There are bonus points for having three times the same number in a diagonal. For each such diagonal of only the number *x*,
  the base score is reduced by 10 * *x*.
* The goal of the player is to place the numbers such that the score is minimized.

## Algorithms

### Actual brute force

Coming soon...

### Dynamic Programming

Coming soon...

## Statictics

Coming soon...
