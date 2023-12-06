const INPUT: &str = include_str!("inputs/6.txt");

fn calculate_number_of_wins(limit: u32, distance: u32) -> u32 {
    /*
    d = h * (l - h)
    h * (l - h) > d
    hl - h^2 > d
    h^2 - hl + d = 0
    solve for h with quadratic formula
    h = (l +/- sqrt(l^2 - 4d)) / 2
     */
    let limit = limit as f32;
    let distance = distance as f32;
    let lo = (limit - (limit.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    let hi = (limit + (limit.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    (hi.ceil() - lo.floor()) as u32 - 1
}

fn parse_ascii(s: &str) -> u32 {
    s.as_bytes()
        .iter()
        .fold(0, |acc, b| acc * 10 + (b & 0xf) as u32)
}

fn parse_ascii_skip_spaces(s: &str) -> u32 {
    s.as_bytes()
        .iter()
        .filter(|&&b| b != b' ')
        .fold(0, |acc, b| acc * 10 + (b & 0xf) as u32)
}

pub fn part1() -> usize {
    let (times, distances) = INPUT.split_once('\n').unwrap();
    let times = times.split_once(':').unwrap().1.trim_start();
    let distances = distances.split_once(':').unwrap().1.trim_start();
    let times = times.split_ascii_whitespace().map(parse_ascii);
    let distances = distances.split_ascii_whitespace().map(parse_ascii);
    times
        .zip(distances)
        .map(|(time, distance)| calculate_number_of_wins(time, distance) as usize)
        .product()
}
pub fn part2() -> usize {
    let (times, distances) = INPUT.split_once('\n').unwrap();
    let times = times.split_once(':').unwrap().1.trim();
    let distances = distances.split_once(':').unwrap().1.trim();
    let time = parse_ascii_skip_spaces(times);
    let distance = parse_ascii_skip_spaces(distances);
    calculate_number_of_wins(time, distance) as usize
}
