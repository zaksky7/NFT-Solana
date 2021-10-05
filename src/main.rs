use std::env;
use std::time::Instant;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod problems;
mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2020;

fn colorize_time(n: f64) -> String {
    let color = if n < 0.5 {
        "\x1b[32m"
    } else if n < 1.0 {
        "\x1b[33m"
    } else {
        "\x1b[31m"
    };
    format!("{}{:.3}{}", color, n, "\x1b[0m")
}

fn run_part(f: Box<dyn Fn(&str) -> String>, input: &str) -> (f64, String) {
    let start = Instant::now();
    let ans = f(input);
    let elapsed = start.elapsed();
    let t: f64 = elapsed.as_millis() as f64 / 1000.0;
    (t, ans)
}

fn run_problem(year: i64, day: i64) -> f64 {
    let contents = problems::get_file_input(year, day, true);

    println!("Day {}", day);
    let (part1, part2) = problems::get_prob(year, day);
    let (t1, ans) = run_part(part1, &contents);
    println!(
        "Part 1: {:>32}  Elapsed time {} seconds",
        ans,
        colorize_time(t1)
    );
    let (t2, ans) = run_part(part2, &contents);
    println!(
        "Part 2: {:>32}  Elapsed time {} seconds",
        ans,
        colorize_time(t2)
    );
    println!();
    t1 + t2
}

fn parse_day(daystr: &str) -> Vec<i64> {
    if daystr.contains("-") {
        let v: Vec<i64> = daystr.splitn(2, "-").map(|x| x.parse().unwrap()).collect();
        (v[0]..=v[1]).collect()
    } else {
        vec![daystr.parse().unwrap()]
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let year = args[0].parse().unwrap();
    let mut days: Vec<i64> = args[1..].iter().flat_map(|x| parse_day(x)).collect();
    if days.is_empty() {
        days = (1..=25).collect();
    }

    let mut total: f64 = 0.0;
    for &day in days.iter() {
        total += run_problem(year, day);
    }
    println!("Total: {:53.3} seconds", total);
}
