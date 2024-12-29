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
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let solutions: HashMap<&str, fn(String)> = HashMap::from([
        ("day1", day1::solve as fn(String)),
        ("day2", day2::solve as fn(String)),
        ("day3", day3::solve as fn(String)),
        ("day4", day4::solve as fn(String)),
        ("day5", day5::solve as fn(String)),
        ("day6", day6::solve as fn(String)),
        ("day7", day7::solve as fn(String)),
        ("day8", day8::solve as fn(String)),
        ("day9", day9::solve as fn(String)),
        ("day10", day10::solve as fn(String)),
        ("day11", day11::solve as fn(String)),
        ("day12", day12::solve as fn(String)),
        ("day13", day13::solve as fn(String)),
        ("day14", day14::solve as fn(String)),
        ("day15", day15::solve as fn(String)),
        ("day16", day16::solve as fn(String)),
        ("day17", day17::solve as fn(String)),
        ("day18", day18::solve as fn(String)),
        ("day19", day19::solve as fn(String)),
        ("day20", day20::solve as fn(String)),
        ("day21", day21::solve as fn(String)),
        ("day22", day22::solve as fn(String)),
        ("day23", day23::solve as fn(String)),
        ("day24", day24::solve as fn(String)),
        ("day25", day25::solve as fn(String)),
    ]);

    let args: Vec<String> = std::env::args().collect();
    // all days
    if args.len() < 2 {
        let mut timings = Vec::new();
        let mut total_time = 0.0;

        for day in 1..26 {
            let input = fs::read_to_string(format!("inputs/day{}.in", day)).unwrap();

            let day_key = format!("day{}", day);
            let solution = solutions.get(&day_key.as_str()).unwrap();
            let start = Instant::now();
            println!("Day {}", day);
            solution(input);
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            timings.push((day, elapsed));
            total_time += elapsed;
        }
        println!("{}", "-".repeat(40));
        println!(
            "{:<10} {:<15} {:<15}",
            "Day", "Time (ms)", "Cumulative (ms)"
        );
        println!("{}", "-".repeat(40));

        let mut cumulative_time = 0.0;
        for (day, elapsed) in &timings {
            cumulative_time += elapsed;
            println!("{:<10} {:<15.2} {:<15.2}", day, elapsed, cumulative_time);
        }

        println!("{}", "-".repeat(40));
        println!("{:<10} {:<15} {:<15.2}", "Total", "", total_time);
        return;
    }
    // specific day
    let input = fs::read_to_string(format!("inputs/{}.in", args[1])).unwrap();
    solutions.get(&args[1].as_str()).unwrap()(input);
}
