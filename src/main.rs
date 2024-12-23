mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
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
        ("day10", day10::solve as fn()),
        ("day11", day11::solve as fn()),
        ("day12", day12::solve as fn()),
        ("day13", day13::solve as fn()),
        ("day14", day14::solve as fn()),
        ("day15", day15::solve as fn()),
        ("day16", day16::solve as fn()),
        ("day17", day17::solve as fn()),
        ("day18", day18::solve as fn()),
        ("day19", day19::solve as fn()),
        ("day20", day20::solve as fn()),
        ("day21", day21::solve as fn()),
        ("day22", day22::solve as fn()),
        ("day23", day23::solve as fn()),
    ]);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    solutions.get(&args[1].as_str()).unwrap()();
}
