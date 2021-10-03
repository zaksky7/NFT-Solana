use std::collections::HashSet;

const W: usize = 50;
const H: usize = 6;

fn process_instr(grid: &mut HashSet<(usize, usize)>, line: &str) {
    if let Ok((a, b)) = scan_fmt!(line, "rect {}x{}", usize, usize) {
        for c in 0..a {
            for r in 0..b {
                grid.insert((r, c));
            }
        }
    } else if let Ok((a, b)) = scan_fmt!(line, "rotate row y={} by {}", usize, usize) {
        *grid = grid
            .iter()
            .map(|(r, c)| (*r, if *r == a { (c + b) % W } else { *c }))
            .collect();
    } else {
        let (a, b) = scan_fmt!(line, "rotate column x={} by {}", usize, usize).unwrap();
        *grid = grid
            .iter()
            .map(|(r, c)| (if *c == a { (r + b) % H } else { *r }, *c))
            .collect();
    }
}

fn lit_pixels(input: &str) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    input
        .lines()
        .for_each(|line| process_instr(&mut result, line));
    result
}

pub fn part1(input: &str) -> usize {
    lit_pixels(input).len()
}

pub fn part2(input: &str) -> String {
    let pix = lit_pixels(input);
    let mut display = vec!["".to_string()];
    for r in 0..H {
        display.push(
            (0..W)
                .map(|c| if pix.contains(&(r, c)) { '#' } else { ' ' })
                .collect(),
        );
    }
    display.push("".to_owned());
    display.join("\n")
}
