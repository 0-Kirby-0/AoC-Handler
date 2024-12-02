# AoC Handler

A small Rust project designed to make solving the [Advent of Code](https://adventofcode.com/) (AoC) programming puzzles smoother and more enjoyable

## What this provides:
- **A convenient scaffold for implementing solutions**
- **A simple interface for testing and running solutions**
- **A completely abstracted input handler that automatically fetches AoC input files (and caches them)**

## How to get working:

### 1. Add the dependency

In your `Cargo.toml`, add the following:

```toml
[dependencies]
aoc_handler = { git = "https://github.com/0-Kirby-0/AoC-Handler" }
```

### 2. Implement a day

To implement a day, implement the `DaySolver` trait for a type of your choosing, usually a zero-size newtype. 
A convenient structure is to create a module for each day, but this is optional. 
Here's an example:

```rust
//in module day_1
use aoc_handler::{DaySolver, SolutionPart};

struct Day;
impl DaySolver for Day {
    // Example: "Count the lines in the input."
    fn part_1(input: &str) -> impl Into<SolutionPart> {
        let parsed = parse(input);   // Invoking external helper functions.
        parsed.iter().count()        // Return is automatically inferred and accepted as the solution.
    }

    // Example: Not yet unlocked.
    fn part_2(input: &str) -> impl Into<SolutionPart> {
        // Returning unit () is valid, and will be treated as "Not finished"
    }

    fn part_1_test_input() -> &'static str {
        r#"Line 1
        Line 2
        Line 3
        Line 4"#
    }

    fn part_1_test_answer() -> impl Into<SolutionPart> {
        4
    }

    // Test input for part 2 is the same as part 1, so left blank.
    // Test answer for part 2 is not yet unlocked, so also left blank.
}
```

### 3. Provide a mapper of solutions

Use the `DayMapper` trait to map solutions for each day:

```rust
mod day_1;
mod day_2;

struct MyMapper;
impl aoc_handler::DayMapper for MyMapper {
    fn map(&self, day: Day) -> aoc_handler::Solver {
        match day {
            1 => day_1::Day.into(),
            2 => day_2::Day.into(),
            0 | 25.. => panic!("Invalid day, AoC only lasts from 1st to 25th of December."),
            _ => unimplemented!("Not yet solved."),
        }
    }
}
```

### 4. Create and use the handler

Create and use the handler in your `main` function.

```rust
fn main() {
    let handler = aoc_handler::Handler::new(2024, MyMapper);
    handler.run(2); 
}
```

The run() function will attempt to run the tests provided for that day. If they pass, or no tests are provided, it will fetch the correct input file and run it on your solution.
