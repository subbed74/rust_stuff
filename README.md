## Sequences
This program returns the number in various integer sequences located at the given indices.  It runs in the terminal and is based off of console commands.

### Commands
**fib** *I1 I2 I3 ...* - Displays the given Fibonacci number.  [A000045](https://oeis.org/A000045)\
**luc** *I1 I2 I3 ...* - Displays the given Lucas number.  [A000032](https://oeis.org/A000032)\
**brady** *I1 I2 I3 ...* - Displays the given Brady number.  [A247698](https://oeis.org/A247698)\
**reca** *I1 I2 I3 ...* - Displays the given Recamán's number.  [A005132](https://oeis.org/A005132)\
**exit** - Exits the program.

## Collatz Calculator
This program takes a given range, then calculates the steps required for every positive integer to reach 1 under the Collatz Conjecture.  Things will be added over time for functionality and only outputting specific items.

## Brainfust
This is yet another brainfuck interpreter written in Rust.  Place the brainfuck code in a file named "main.bf", and run the program with the file in the same directory.  It takes input during runtime, but it will only take the first character from a string if more than one character is given.

## MUSIC THEORY - Twelve Tone Matrix Creator
This is a simple little program that calculates a 12-tone matrix based off the prime row given.  Input is done by writing the 12 pitch classes as numbers 0-11, though it will print out "T" and "E" for 10 and 11 respectively.  That was done primarily for alignment purposes.
