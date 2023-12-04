use rustc_hash::{FxHashMap, FxHashSet};

const INPUT: &str = include_str!("inputs/4.txt");
pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(": ").unwrap();
            let (winning, mine) = l.split_once(" | ").unwrap();
            let winning: FxHashSet<&str> = winning.split_whitespace().collect();
            mine.split_whitespace()
                .filter(|n| winning.contains(n))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum()
}

pub fn part2() -> usize {
    let mut cards: Vec<_> = INPUT
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(": ").unwrap();
            let (winning, mine) = l.split_once(" | ").unwrap();
            let winning: FxHashSet<&str> = winning.split_whitespace().collect();
            mine.split_whitespace()
                .filter(|n| winning.contains(n))
                .count()
        })
        .collect();
    let mut amounts_per_card = vec![1; cards.len()];

    for (i, &matches) in cards.iter().enumerate() {
        // for each copy of this card we have...
        for _ in 0..amounts_per_card[i] {
            // for each win, add a copy of subsequent cards
            for j in 0..matches {
                amounts_per_card[i + j + 1] += 1;
            }
        }
    }
    amounts_per_card.iter().sum()
}
