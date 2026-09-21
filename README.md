# Exercism Rust Solutions

A collection of solutions for exercises from the [Exercism](https://exercism.org/) Rust track. This repository serves as a personal log of problem-solving, algorithm implementation, and idiomatic Rust practice.

## 📁 Repository Structure

Each exercise is a standalone Cargo crate with the solution in `src/` and the provided test suite in `tests/`:

- **`anagram`** — Find every anagram of a word among a list of candidates.
- **`armstrong-numbers`** — Check if a number is an Armstrong number.
- **`bottle-song`** — Generate the lyrics of "Ten Green Bottles".
- **`clock`** — A 24-hour clock that handles minute arithmetic and wrapping.
- **`difference-of-squares`** — Difference between the sum of squares and the square of the sum.
- **`flower-field`** — Annotate a field with the count of flowers adjacent to each square.
- **`gigasecond`** — Determine the moment one gigasecond after a given instant.
- **`grains`** — Calculate the number of grains of wheat on a chessboard.
- **`hello-world`** — The classic introductory exercise.
- **`leap`** — Determine whether a year is a leap year.
- **`luhn`** — Validate numbers using the Luhn checksum formula.
- **`nth-prime`** — Find the nth prime number.
- **`prime-factors`** — Compute the prime factors of a given natural number.
- **`reverse-string`** — Reverse a string, including multi-byte grapheme clusters.
- **`space-age`** — Calculate an age in years on each planet of the solar system.
- **`sublist`** — Determine whether a list is a sublist, superlist, equal, or unequal to another.

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
