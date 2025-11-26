# Advent of Code 2025

My [Advent of Code 2025](https://adventofcode.com/2025) solutions in the Rust programming language. This repository holds a separate Rust project for each day.

## Running the Code

To run the code for a given day use the following commands:

`cargo run -r -p day12 --bin part1`

`cargo run -r -p day12 --bin part2`

## Running Unit Tests

To run unit tests for just one day use the `-p` option. e.g.:

`cargo test -p day05`

You can also limit tests to just one part by adding `part1` or `part2` to the end of the command. e.g.:

`cargo test -p day07 part2`

## Running Benchmarks

To run benchmarks for one day use:

`cargo bench -p day16`
