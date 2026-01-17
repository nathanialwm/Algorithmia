use std::env;
use std::fs;
use Algorithmia::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        eprintln!("Usage: {} <name>", args[1]);
        return;
    }
    let program = &args[1];
    println!("Program number: {}", program);
    match program.as_str() {
        "1.1" => {
            let notes = fs::read_to_string("./notes/1.1.txt")
                .expect("Failed to read notes file");
            let potions = total_potions_1(notes.to_string());
            println!("Total potions: {}", potions);
        }
        "1.2" => {
            let notes = fs::read_to_string("./notes/1.2.txt")
                .expect("Failed to read notes file");
            let potions = total_potions_2(notes.to_string());
            println!("Total potions: {}", potions);
        }
        "1.3" => {
            let notes = fs::read_to_string("./notes/1.3.txt")
                .expect("Failed to read notes file");
            let potions = total_potions_3(notes.to_string());
            println!("Total potions: {}", potions);
        }
        "2.1" => {
            let notes = fs::read_to_string("./notes/2.1.txt")
                .expect("Failed to read notes file");
            let result = runic_words_1(notes.to_string());
            println!("Runic words result: {}", result);
        }
        _ => {
            eprintln!("Unknown program: {}", program);
        }
    }
}
