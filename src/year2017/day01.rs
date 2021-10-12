pub fn part1(input: &str) -> u32 {
    let s = input.as_bytes();
    (0..s.len())
        .filter_map(|i| (s[i] == s[(i + 1) % s.len()]).then(|| (s[i] - b'0') as u32))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let s = input.as_bytes();
    (0..s.len())
        .filter_map(|i| (s[i] == s[(i + s.len() / 2) % s.len()]).then(|| (s[i] - b'0') as u32))
        .sum()
}
