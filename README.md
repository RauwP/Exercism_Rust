# Exercism Rust Solutions

A collection of solutions for exercises from the [Exercism](https://exercism.org/) Rust track. This repository serves as a personal log of problem-solving, algorithm implementation, and idiomatic Rust practice.

## 📁 Repository Structure

Each exercise is a standalone Cargo crate with the solution in `src/` and the provided test suite in `tests/`, numbered in the order I solved them:

- **`01-hello-world`** - The classic introductory exercise.
- **`02-gigasecond`** - Determine the moment one gigasecond after a given instant.
- **`03-reverse-string`** - Reverse a string, including multi-byte grapheme clusters.
- **`04-clock`** - A 24-hour clock that handles minute arithmetic and wrapping.
- **`05-anagram`** - Find every anagram of a word among a list of candidates.
- **`06-space-age`** - Calculate an age in years on each planet of the solar system.
- **`07-sublist`** - Determine whether a list is a sublist, superlist, equal, or unequal to another.
- **`08-flower-field`** - Annotate a field with the count of flowers adjacent to each square.
- **`09-luhn`** - Validate numbers using the Luhn checksum formula.
- **`10-armstrong-numbers`** - Check if a number is an Armstrong number.
- **`11-bottle-song`** - Generate the lyrics of "Ten Green Bottles".
- **`12-difference-of-squares`** - Difference between the sum of squares and the square of the sum.
- **`13-grains`** - Calculate the number of grains of wheat on a chessboard.
- **`14-prime-factors`** - Compute the prime factors of a given natural number.
- **`15-leap`** - Determine whether a year is a leap year.
- **`16-nth-prime`** - Find the nth prime number.
- **`17-proverb`** - Generate the "For Want of a Nail" chain proverb for a list of inputs.
- **`18-raindrops`** - Convert a number to its corresponding raindrop sounds.
- **`19-sum-of-multiples`** - Sum the multiples of a set of numbers, up to a limit.
- **`20-bob`** - Respond to remarks depending on their tone.
- **`21-high-scores`** - Track a player's list of scores and derive stats from it.
- **`22-matching-brackets`** - Make sure the brackets and braces all match.
- **`23-collatz-conjecture`** - Calculate the number of steps from a number to 1 using the collatz conjecture rules.
- **`24-series`** - return all series of length n from a string of digits.
- **`25-kindergarten-garden`** - return which flowers each child decided to plant.

## 🛠️ Running Tests

Each exercise is its own crate. To run the tests for one, navigate into its directory and run:

```bash
cargo test
```

Exercism ships most tests marked `#[ignore]` so they can be enabled one at a time while working through an exercise. To run the complete suite for an exercise:

```bash
cargo test -- --include-ignored
```

To check that a solution compiles without running the tests:

```bash
cargo check
```
