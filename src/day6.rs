const INPUT: &str = include_str!("inputs/6.txt");

type N = u64;

fn calculate_distance(time_held: N, time_limit: N) -> N {
    if time_limit <= time_held {
        0
    } else {
        time_held * (time_limit - time_held)
    }
}

pub fn part1() -> usize {
    let (times, distances) = INPUT.split_once('\n').unwrap();
    let times = times.split_once(':').unwrap().1.trim();
    let distances = distances.split_once(':').unwrap().1.trim();
    let times = times
        .split_ascii_whitespace()
        .map(|t| t.parse::<N>().unwrap());
    let distances = distances
        .split_ascii_whitespace()
        .map(|t| t.parse::<N>().unwrap());
    times
        .zip(distances)
        .map(|(time, distance)| {
            (1..time)
                .filter(|hold| calculate_distance(*hold, time) > distance)
                .count()
        })
        .product()
}
pub fn part2() -> usize {
    let (times, distances) = INPUT.split_once('\n').unwrap();
    let times = times.split_once(':').unwrap().1.trim();
    let distances = distances.split_once(':').unwrap().1.trim();
    let time: u64 = times.split_ascii_whitespace().fold(0, |acc, x| {
        let l = x.len();
        (acc * 10u64.pow(l as u32)) + x.parse::<N>().unwrap()
    });
    let distance: u64 = distances.split_ascii_whitespace().fold(0, |acc, x| {
        let l = x.len();
        (acc * 10u64.pow(l as u32)) + x.parse::<N>().unwrap()
    });
    (1..time)
        .filter(|hold| calculate_distance(*hold, time) > distance)
        .count()
}
