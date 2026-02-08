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

struct DayImpl;
impl DaySolver for DayImpl {
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
        "Line 1
        Line 2
        Line 3
        Line 4"
    }

    fn part_1_test_answer() -> impl Into<SolutionPart> {
        4
    }

    // Test input for part 2 is the same as part 1, so left blank.
    // Test answer for part 2 is not yet unlocked, so also left blank.
}
```

### 3. Provide a mapper of solutions

Provide a function (or closure) that maps a `(Year, Day)` to a solver:

```rust
use aoc_handler::{Year, Day, Solver}

mod day_1;
mod day_2;

fn map(year: Year, day: Day) -> Option<Solver> {
    match day {
        1 => day_1::DayImpl.wrap(),
        2 => day_2::DayImpl.wrap(),
        _ => None,
    }
}
```

### 4. Create and use the handler

Create and use the handler in your `main` function.

```rust
fn main() {
    let handler = aoc_handler::Handler::new(&map);
    
    handler.run_most_recent_part(2024);
}
```

The API is split into **check** and **run** families:

- **check\_***: runs test inputs and compares to test answers.  
- **run\_***: does the check first, then (if it passes or no tests are provided) fetches the real AoC input and executes (and times) the solution.

**Common highlights and use cases:**
- **Solving in order:**  
  - `run_most_recent_part(year)` is the default “keep going” option — it finds your latest implemented day and highest implemented part.  
  - `run_most_recent_day(year)` is similar but runs both parts for that day (if implemented).
- **Sanity check after changes:**  
  - `check_year(year)` verifies all test cases for a year.
- **Benchmarking/optimizing:**  
  - `run_year(year)` re-runs the full year on real inputs to compare performance.

**Other variants you may need:**
- `check_day`, `run_day` — target a single day.
- `check_part`, `run_part` — target a specific part.
- `check_year_range`, `run_year_range` — bulk operations across multiple years.
