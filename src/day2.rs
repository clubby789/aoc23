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
            match col.as_bytes()[0] {
                b'r' => red = red.max(n),
                b'g' => blue = blue.max(n),
                b'b' => green = green.max(n),
                _ => ()
            }
        }
    }
    (red, green, blue)
}

pub fn part1() -> usize {
    let mut sum = 0;
    'games: for (i, game) in INPUT.lines().enumerate() {
        let pulls = game.split_once(": ").unwrap().1;
        for pull in pulls.split("; ") {
            for cube in pull.split(", ") {
                let (n, col) = cube.split_once(" ").unwrap();
                let n: u8 = n.parse().unwrap();
                match col.as_bytes()[0] {
                    b'r' if n > 12 => continue 'games,
                    b'g' if n > 13 => continue 'games,
                    b'b' if n > 14 => continue 'games,
                    _ => ()
                }
            }
        }
        sum += i + 1;
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