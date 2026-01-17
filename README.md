# Kingdom of Algorithmia

My solutions for the coding quests on [everybody.codes](https://everybody.codes) — Kingdom of Algorithmia edition. All solutions are written in Rust.

## Table of Contents

- [Progress](#progress)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Quests Overview](#quests-overview)
- [Key Rust Concepts](#key-rust-concepts)
- [License](#license)

## Progress

| Quest | Status | Tasks Completed |
|-------|--------|-----------------|
| Quest 1 | Completed | 3/3 |
| Quest 2 | In Progress | 1/? |
| Quests 3-20 | Not Started | 0 |

**Total tasks completed:** 4

## Installation

Ensure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs/).

Clone and build the project:
```bash
git clone <your-repo-url>
cd Algorithmia
cargo build --release
```

## Usage

### Running a Task

Run a specific task by passing the quest and task number:
```bash
cargo run <quest.task>
```

Examples:
```bash
cargo run 1.1    # Run Quest 1, Task 1
cargo run 1.2    # Run Quest 1, Task 2
cargo run 2.1    # Run Quest 2, Task 1
```

### Output Format
```
Program number: 1.1
Total potions: 1328
```

## Project Structure

```
Algorithmia/
├── src/
│   ├── main.rs          # Entry point, argument parsing
│   ├── lib.rs           # Library code, module exports
│   └── quests/
│       ├── mod.rs       # Quest module declarations
│       ├── 1.rs         # Quest 1 solutions
│       └── 2.rs         # Quest 2 solutions
├── notes/               # Input files for each task
│   ├── 1.1.txt
│   ├── 1.2.txt
│   ├── 1.3.txt
│   └── 2.1.txt
├── Cargo.toml           # Project manifest
└── README.md
```

### Module Organization
- **main.rs**: Binary crate - handles CLI interaction and program flow
- **lib.rs**: Library crate - exports quest solution functions
- **quests/**: Contains solution modules for each quest

## Quests Overview

### Quest 1: Potion Brewing
Calculate total potions needed based on note sequences. Each task introduces additional complexity in the calculation rules.

### Quest 2: Runic Words
Find and count occurrences of runic words (substrings) within ancient texts using pattern matching techniques.

## Key Rust Concepts

This project demonstrates several important Rust concepts:

### String Processing
```rust
notes.lines()
    .next().unwrap_or("")
    .strip_prefix("WORDS:").unwrap_or("")
```
Chaining iterator methods and handling `Option` types for safe string manipulation.

### Collections
```rust
let mut set: HashSet<String> = HashSet::new();
set.insert(word.to_string());
```
Using `HashSet` for efficient lookups when searching for word matches.

### Pattern Matching
```rust
match program.as_str() {
    "1.1" => { /* ... */ }
    "2.1" => { /* ... */ }
    _ => eprintln!("Unknown program"),
}
```
Rust's powerful `match` expressions for control flow.

### Iterator Patterns
```rust
for word in words.split(",") {
    let trimmed = word.trim();
    set.insert(trimmed.to_string());
}
```
Functional programming style with iterators and string methods.

## License

Unlicensed

## About

Kingdom of Algorithmia is a series of programming challenges from [everybody.codes](https://everybody.codes). Each quest presents algorithmic problems of increasing difficulty, with multiple tasks per quest.

---

**Built with Rust**
