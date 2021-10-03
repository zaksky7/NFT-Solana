use std::fs;
use std::path::Path;

use crate::year2016::assembunny;

pub fn part1(input: &str) -> Option<i64> {
    let ssim = assembunny::parse_instrs(input);
    (0..)
        .filter(|i| {
            let mut sim = ssim.clone();
            sim.regs[0] = *i;
            sim.take(10)
                .zip([0, 1].iter().cycle())
                .all(|(a, b)| a == *b)
        })
        .next()
}

pub fn part2(_: &str) -> String {
    let sim = assembunny::parse_instrs(
        fs::read_to_string(Path::new("inputs/2016/bonuschallenge.txt"))
            .unwrap()
            .trim(),
    );
    let output: String = sim.into_iter().map(|x| x as u8 as char).collect();
    crate::year2016::day08::part2(&output)
}
