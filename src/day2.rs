const INPUT: &str = include_str!("inputs/2.txt");

// Given a single game, find the minimum number of cubes that must have been in the bag
fn min_num_cubes(game: &str) -> (u8, u8, u8) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let pulls = game.split_once(": ").unwrap().1;
    for pull in pulls.split("; ") {
        for cube in pull.split(", ") {
            let (n, col) = cube.split_once(" ").unwrap();
            let n: u8 = n.parse().unwrap();
            match col {
                "red" => red = red.max(n),
                "blue" => blue = blue.max(n),
                "green" => green = green.max(n),
                _ => ()
            }
        }
    }
    (red, green, blue)
}

pub fn part1() -> usize {
    let mut sum = 0;
    for (i, game) in INPUT.lines().enumerate() {
        let (red, green, blue) = min_num_cubes(game);
        if red <= 12 && green <= 13 && blue <= 14 {
            sum += i + 1;
        }
    }
    sum
}
pub fn part2() -> usize {
    let mut sum = 0;
    for game in INPUT.lines() {
        let (red, green, blue) = min_num_cubes(game);
        sum += red as usize * green as usize * blue as usize
    }
    sum
}