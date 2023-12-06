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

pub fn part1() -> usize {
    let (times, distances) = INPUT.split_once('\n').unwrap();
    let times = times.split_once(':').unwrap().1.trim();
    let distances = distances.split_once(':').unwrap().1.trim();
    let times = times
        .split_ascii_whitespace()
        .map(|t| t.parse::<u32>().ok().unwrap());
    let distances = distances
        .split_ascii_whitespace()
        .map(|t| t.parse::<u32>().ok().unwrap());
    times
        .zip(distances)
        .map(|(time, distance)| calculate_number_of_wins(time, distance) as usize)
        .product()
}
pub fn part2() -> usize {
    let (times, distances) = INPUT.split_once('\n').unwrap();
    let times = times.split_once(':').unwrap().1.trim();
    let distances = distances.split_once(':').unwrap().1.trim();
    let time = times.split_ascii_whitespace().fold(0, |acc, x| {
        let l = x.len();
        (acc * 10u32.pow(l as u32)) + x.parse::<u32>().ok().unwrap()
    });
    let distance: u32 = distances.split_ascii_whitespace().fold(0, |acc, x| {
        let l = x.len();
        (acc * 10u32.pow(l as u32)) + x.parse::<u32>().ok().unwrap()
    });
    calculate_number_of_wins(time, distance) as usize
}
