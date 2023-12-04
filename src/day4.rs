use rustc_hash::{FxHashMap, FxHashSet};

const INPUT: &str = include_str!("inputs/4.txt");

fn parse_two_byte_num(hi: u8, lo: u8) -> u8 {
    let hi = if hi == b' ' { 0 } else { hi - b'0' };
    let lo = lo - b'0';
    hi * 10 + lo
}

fn matches_for_card(card: &str) -> usize {
    let (_, l) = card.split_once(": ").unwrap();
    let (winning, mine) = l.split_once(" | ").unwrap();
    let mut winning_numbers = [false; 100];
    for w in winning.as_bytes().chunks(3) {
        let &[hi, lo, ..] = w else { unreachable!() };
        let n = parse_two_byte_num(hi, lo) as usize;
        winning_numbers[n] = true;
    }
    let mut sum = 0;
    mine.as_bytes()
        .chunks(3)
        .filter(|m| {
            let &&[hi, lo, ..] = m else { unreachable!() };
            let n = parse_two_byte_num(hi, lo) as usize;
            winning_numbers[n]
        })
        .count()
}

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let m = matches_for_card(l);
            if m > 0 {
                2usize.pow((m - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

pub fn part2() -> usize {
    let mut cards: Vec<_> = INPUT.lines().map(matches_for_card).collect();
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
