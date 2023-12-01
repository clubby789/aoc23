const INPUT: &str = include_str!("inputs/1.txt");

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            (
                l.as_bytes().iter().find(|b| b.is_ascii_digit())
                    .map(|i| i - b'0')
                    .unwrap(),
                l.as_bytes().iter().rev().find(|b| b.is_ascii_digit())
                    .map(|i| i - b'0')
                    .unwrap(),
            )
        })
        .map(|(l, r)| (l * 10 + r) as usize)
        .sum()
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .map(|l| {
            fn num(s: &str) -> Option<usize> {
                static NUMS: [(u8, &str); 9] = [
                    (b'1', "one"),
                    (b'2', "two"),
                    (b'3', "three"),
                    (b'4', "four"),
                    (b'5', "five"),
                    (b'6', "six"),
                    (b'7', "seven"),
                    (b'8', "eight"),
                    (b'9', "nine"),
                ];
                NUMS.iter()
                    .enumerate()
                    .find(|(_, &(n1, n2))| s.as_bytes().get(0).copied() == Some(n1) || s.starts_with(n2))
                    .map(|(i, _)| i + 1)
            }
            let n1 = (0..l.len()).find_map(|n| num(&l[n..])).unwrap();
            let n2 = (0..=l.len()).rev().find_map(|n| num(&l[n..])).unwrap();
            n1 * 10 + n2
        })
        .sum()
}
