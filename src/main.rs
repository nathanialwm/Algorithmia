use Algorithmia::*;
use std::collections::HashMap;
use std::env;
use std::fs;

type SolverFn = fn(String) -> i32;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <quest.task>", args[0]);
        return;
    }
    let program = &args[1];

    let solvers: HashMap<&str, SolverFn> = HashMap::from([
        ("1.1", total_potions_1 as SolverFn),
        ("1.2", total_potions_2 as SolverFn),
        ("1.3", total_potions_3 as SolverFn),
        ("2.1", runic_words_1 as SolverFn),
        ("2.2", runic_words_2 as SolverFn),
        ("2.3", runic_words_3 as SolverFn),
        ("3.1", mining_1 as SolverFn),
    ]);

    if let Some(solver) = solvers.get(program.as_str()) {
        let path = format!("./notes/{}.txt", program);
        let notes = fs::read_to_string(&path).expect("Failed to read notes file");
        let result = solver(notes);
        println!("Program: {}\nResult: {}", program, result);
    } else {
        eprintln!("Unknown program: {}", program);
    }
}
