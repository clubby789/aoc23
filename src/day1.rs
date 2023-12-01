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
                static NUMS: [(&str, &str); 9] = [
                    ("1", "one"),
                    ("2", "two"),
                    ("3", "three"),
                    ("4", "four"),
                    ("5", "five"),
                    ("6", "six"),
                    ("7", "seven"),
                    ("8", "eight"),
                    ("9", "nine"),
                ];
                NUMS.iter()
                    .enumerate()
                    .find(|(_, &(n1, n2))| s.starts_with(n1) || s.starts_with(n2))
                    .map(|(i, _)| i + 1)
            }
            let n1 = (0..l.len()).find_map(|n| num(&l[n..])).unwrap();
            let n2 = (0..=l.len()).rev().find_map(|n| num(&l[n..])).unwrap();
            n1 * 10 + n2
        })
        .sum()
}
