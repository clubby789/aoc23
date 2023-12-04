const INPUT: &str = include_str!("inputs/4.txt");

fn parse_two_byte_num(hi: u8, lo: u8) -> u8 {
    let hi = if hi == b' ' { 0 } else { hi - b'0' };
    let lo = lo - b'0';
    hi * 10 + lo
}

// Returns the length of the Card ...: prefix, and the length of the 'winning numbers' section
// Could be hardcoded, but sort of cheating
fn amount_to_skip() -> (usize, usize) {
    let s1 = std::hint::black_box(INPUT).find(":").unwrap() + 2;
    let s2 = std::hint::black_box(INPUT).find('|').unwrap() + 2 - s1;
    (s1, s2)
}

fn matches_for_card(skips: (usize, usize), card: &str) -> usize {
    let (_, l) = card.split_at(skips.0);
    let (winning, mine) = l.split_at(skips.1);
    // remove ' | '
    let winning = &winning[..winning.len() - 3];
    let mut winning_numbers = [false; 100];
    for w in winning.as_bytes().chunks(3) {
        let &[hi, lo, ..] = w else { unreachable!() };
        let n = parse_two_byte_num(hi, lo) as usize;
        winning_numbers[n] = true;
    }
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
    let skips = amount_to_skip();
    INPUT
        .lines()
        .map(|l| {
            let m = matches_for_card(skips, l);
            if m > 0 {
                2usize.pow((m - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

pub fn part2() -> usize {
    let skips = amount_to_skip();
    let mut cards: Vec<_> = INPUT.lines().map(|l| matches_for_card(skips, l)).collect();
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
