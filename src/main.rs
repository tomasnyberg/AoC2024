mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::collections::HashMap;

fn main() {
    let solutions: HashMap<&str, fn()> = HashMap::from([
        ("day1", day1::solve as fn()),
        ("day2", day2::solve as fn()),
        ("day3", day3::solve as fn()),
        ("day4", day4::solve as fn()),
        ("day5", day5::solve as fn()),
        ("day6", day6::solve as fn()),
        ("day7", day7::solve as fn()),
        ("day8", day8::solve as fn()),
        ("day9", day9::solve as fn()),
    ]);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    solutions.get(&args[1].as_str()).unwrap()();
}
