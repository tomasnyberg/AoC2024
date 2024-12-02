mod day1;
mod day2;
mod day3;

use std::collections::HashMap;

fn main() {
    let solutions: HashMap<&str, fn()> = HashMap::from([
        ("day1", day1::solve as fn()),
        ("day2", day2::solve as fn()),
        ("day3", day3::solve as fn()),
    ]);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    solutions.get(&args[1].as_str()).unwrap()();
}
