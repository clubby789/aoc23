const INPUT: &str = include_str!("inputs/9.txt");

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let nums: Vec<isize> = l.split(" ").map(|n| n.parse().ok().unwrap()).collect();
            let mut diffs: Vec<Vec<isize>> = vec![nums.windows(2).map(|w| w[1] - w[0]).collect()];

            loop {
                let ld = diffs.last().unwrap();
                if ld.iter().all(|&d| d == 0) {
                    break;
                }
                diffs.push(ld.windows(2).map(|w| w[1] - w[0]).collect());
            }
            let diff_sums = diffs
                .into_iter()
                .rev()
                .map(|d| *d.last().unwrap())
                .sum::<isize>();
            diff_sums + nums.last().unwrap()
        })
        .sum::<isize>()
        .try_into()
        .unwrap()
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let nums: Vec<isize> = l.split(" ").map(|n| n.parse().ok().unwrap()).collect();
            let mut diffs: Vec<Vec<isize>> = vec![nums.windows(2).map(|w| w[1] - w[0]).collect()];

            loop {
                let ld = diffs.last().unwrap();
                if ld.iter().all(|&d| d == 0) {
                    break;
                }
                diffs.push(ld.windows(2).map(|w| w[1] - w[0]).collect());
            }
            let start_diff = diffs
                .into_iter()
                .rev()
                .fold(0isize, |acc, diff| diff[0] - acc);
            nums[0] - start_diff
        })
        .sum::<isize>()
        .try_into()
        .unwrap()
}
