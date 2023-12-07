use std::cmp::Ordering;

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Hand<const JOKER: bool>([Card; 5], u64);

impl<const JOKER: bool> PartialOrd for Hand<JOKER> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<const JOKER: bool> Ord for Hand<JOKER> {
    fn cmp(&self, other: &Self) -> Ordering {
        let t1 = self.hand_type();
        let t2 = other.hand_type();
        match t1.cmp(&t2) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.0.cmp(&other.0),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

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

impl<const JOKER: bool> Hand<JOKER> {
    pub fn hand_type(&self) -> Type {
        // bitset of encountered cards
        let mut encountered = BitSet::new();
        let mut jokers = 0;
        let mut cards = self.0;
        cards.sort_unstable();
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
                let midp = cards
                    .windows(2)
                    .enumerate()
                    .find_map(|(i, w)| if w[0] != w[1] { Some(i + 1) } else { None })
                    .unwrap();
                match midp {
                    1 | 4 => Type::FourOfAKind,
                    _ => {
                        debug_assert!(matches!(midp, 2 | 3));
                        Type::FullHouse
                    }
                }
            }
            3 => {
                let mut p1 = None;
                let mut p2 = None;
                for (i, w) in cards.windows(2).enumerate() {
                    if w[0] != w[1] {
                        if p1.is_none() {
                            p1 = Some(i + 1)
                        } else if p2.is_none() {
                            p2 = Some(i + 1)
                        } else {
                            break;
                        }
                    }
                }
                let p1 = p1.unwrap();
                let p2 = p2.unwrap();
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

                        u => unreachable!("{u:?}"),
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
}

#[derive(PartialOrd, PartialEq, Debug, Ord, Eq)]
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
    inp.lines()
        .map(|l| {
            let (hand, bet) = l.split_once(' ').unwrap();
            let hand = hand.as_bytes();
            assert_eq!(hand.len(), 5);
            let hand = [0, 1, 2, 3, 4].map(|i| match hand[i] {
                b'2' => Card::Two,
                b'3' => Card::Three,
                b'4' => Card::Four,
                b'5' => Card::Five,
                b'6' => Card::Six,
                b'7' => Card::Seven,
                b'8' => Card::Eight,
                b'9' => Card::Nine,
                b'T' => Card::Ten,
                b'J' if JOKER => Card::Joker,
                b'J' => Card::Jack,
                b'Q' => Card::Queen,
                b'K' => Card::King,
                b'A' => Card::Ace,
                _ => unreachable!(),
            });
            let bet = bet.parse().unwrap();
            Hand(hand, bet)
        })
        .collect()
}

pub fn part1() -> usize {
    let mut bets = parse_input::<false>(INPUT);
    // sort from weakest hand first
    bets.sort();
    bets.into_iter()
        .enumerate()
        .map(|(rank, bet)| (rank + 1) * bet.1 as usize)
        .sum()
}

pub fn part2() -> usize {
    let mut bets = parse_input::<true>(INPUT);
    // sort from weakest hand first
    bets.sort();
    bets.into_iter()
        .enumerate()
        .map(|(rank, bet)| (rank + 1) * bet.1 as usize)
        .sum()
}
