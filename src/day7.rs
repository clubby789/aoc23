const INPUT: &str = include_str!("inputs/7.txt");

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
enum Card {
    Joker = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

const CARD_ARR: [Card; 15] = [
    Card::Joker,
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::Jack,
    Card::Queen,
    Card::Queen,
    Card::King,
    Card::Ace,
];

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Hand<const JOKER: bool>(Type, [Card; 5], u32);

struct BitSet(u16);
impl BitSet {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn set(&mut self, bit: usize) {
        self.0 |= 1 << bit;
    }

    pub fn get(&mut self, bit: usize) -> bool {
        (self.0 >> bit) & 1 == 1
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

fn midpoints<const N: usize>(cards: &[Card; 5]) -> [usize; N] {
    let mut pos = 0;
    std::array::from_fn(|_| {
        while cards[pos] == cards[pos + 1] {
            pos += 1
        }
        pos += 1;
        pos
    })
}

fn hand_type<const JOKER: bool>(mut cards: [Card; 5]) -> Type {
    // bitset of encountered cards
    let mut encountered = BitSet::new();
    let mut jokers = 0;
    for c in cards {
        if JOKER && c == Card::Joker {
            jokers += 1;
        }
        encountered.set(c as usize);
    }
    let uniques = encountered.len();
    match uniques {
        1 => Type::FiveOfAKind,
        2 if JOKER && jokers > 0 => Type::FiveOfAKind,
        2 => {
            cards.sort_unstable();
            let [midp] = midpoints(&cards);
            match midp {
                1 | 4 => Type::FourOfAKind,
                _ => {
                    debug_assert!(matches!(midp, 2 | 3));
                    Type::FullHouse
                }
            }
        }
        3 => {
            cards.sort_unstable();
            let [p1, p2] = midpoints(&cards);
            if JOKER && jokers > 0 {
                match (p1, p2, jokers) {
                    // AAABJ -> AAABA
                    // AAAJB |
                    // JAAAB |
                    // BJAAA |
                    (3, 4, 1) | (1, 4, 1) | (1, 2, 1) => Type::FourOfAKind,
                    // AABJJ -> AABAA
                    // AAJJB |
                    // JJAAB |
                    // JJBAA |
                    (2, 3, 2) | (2, 4, 2) => Type::FourOfAKind,
                    // ABJJJ -> ABBBB
                    // AJJJB |
                    // JJJAB |
                    (1, 2, 3) | (1, 4, 3) | (3, 4, 3) => Type::FourOfAKind,

                    // AABBJ -> AABBB
                    // AAJBB |
                    // JAABB |
                    (2, 4, 1) | (2, 3, 1) | (1, 3, 1) => Type::FullHouse,

                    _ => unreachable!(),
                }
            } else {
                match (p1, p2) {
                    // ABCCC
                    (1, 2) => Type::ThreeOfAKind,
                    // ABBBC
                    (1, 4) => Type::ThreeOfAKind,
                    // AAABC
                    (3, 4) => Type::ThreeOfAKind,
                    // AABBC
                    (2, 4) => Type::TwoPair,
                    // ABBCC
                    (1, 3) => Type::TwoPair,
                    // AABCC
                    (2, 3) => Type::TwoPair,
                    _ => unreachable!(),
                }
            }
        }
        // ABCJJ -> ABCCC
        // ABBCJ -> ABBCB
        4 if JOKER && jokers > 0 => Type::ThreeOfAKind,
        4 => Type::OnePair,
        5 if JOKER && jokers > 0 => Type::OnePair,
        _ => Type::HighCard,
    }
}

#[derive(PartialOrd, PartialEq, Debug, Ord, Eq, Copy, Clone)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_input<const JOKER: bool>(inp: &str) -> Vec<Hand<JOKER>> {
    let mut result = Vec::with_capacity(inp.len() / 9);
    let src = inp.as_bytes();
    let mut pos = 0;
    while pos < inp.len() {
        let hand: &[u8; 5] = &src[pos..pos + 5].try_into().unwrap();
        let hand = hand.map(|b| match b {
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::Ten,
            b'J' => {
                if JOKER {
                    Card::Joker
                } else {
                    Card::Jack
                }
            }
            b'Q' => Card::Queen,
            b'K' => Card::King,
            b'A' => Card::Ace,
            _ => unreachable!(),
        });
        // skip hand and space
        pos += 6;
        let mut bet = 0;
        while pos < inp.len() && src[pos] != b'\n' {
            bet = bet * 10 + (src[pos] & 0xf) as u32;
            pos += 1;
        }
        pos += 1;
        let typ = hand_type::<JOKER>(hand);
        result.push(Hand(typ, hand, bet))
    }
    result
}

pub fn part1() -> usize {
    let mut bets = parse_input::<false>(INPUT);
    // sort from weakest hand first
    bets.sort_unstable();
    bets.into_iter()
        .enumerate()
        .map(|(rank, bet)| (rank + 1) * bet.2 as usize)
        .sum()
}

pub fn part2() -> usize {
    let mut bets = parse_input::<true>(INPUT);
    // sort from weakest hand first
    bets.sort_unstable();
    bets.into_iter()
        .enumerate()
        .map(|(rank, bet)| (rank + 1) * bet.2 as usize)
        .sum()
}
