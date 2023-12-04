const INPUT: &str = include_str!("inputs/4.txt");

fn parse_two_byte_num(hi: u8, lo: u8) -> u8 {
    // hi may be a space, in which case this wraps to 240
    // use a mask instead to make space become 0
    let hi = (hi - b'0') & 0b1111;
    let lo = lo - b'0';
    hi * 10 + lo
}

// Returns the length of the Card ...: prefix
// Could be hardcoded, but sort of cheating
// forcing inlining improves perf by a couple of us
#[inline(always)]
fn amount_to_skip() -> usize {
    std::hint::black_box(INPUT).find(":").unwrap() + 2
}

fn matches_for_card(skip: usize, card: &[u8]) -> usize {
    let (_, l) = card.split_at(skip);
    let mut pos = 0;
    let mut winning_numbers = [false; 100];
    while let Some(&[hi, lo]) = l.get(pos..pos + 2) {
        if hi == b'|' {
            pos += 2;
            break;
        }
        winning_numbers[parse_two_byte_num(hi, lo) as usize] = true;
        pos += 3;
    }
    let mut sum = 0;
    while let Some(&[hi, lo]) = l.get(pos..pos + 2) {
        if winning_numbers[parse_two_byte_num(hi, lo) as usize] {
            sum += 1;
        }
        pos += 3;
    }
    sum
}

pub fn part1() -> usize {
    let skip = amount_to_skip();
    INPUT
        .lines()
        .map(|l| {
            let m = matches_for_card(skip, l.as_bytes());
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
    let mut amounts_per_card = [1; 256];

    let mut cards = INPUT.lines().map(|l| matches_for_card(skips, l.as_bytes()));
    for (i, matches) in cards.enumerate() {
        let copies = amounts_per_card[i];
        // for each match, add a copy of subsequent cards for each copy of this card
        for j in 0..matches {
            amounts_per_card[i + j + 1] += copies;
        }
    }
    amounts_per_card.iter().sum()
}
