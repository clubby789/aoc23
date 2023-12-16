// const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

const INPUT: &str = include_str!("inputs/15.txt");

fn hash_string(s: &str) -> usize {
    s.bytes().fold(0, |acc, b| ((acc + b as usize) * 17) % 256)
}

pub fn part1() -> usize {
    INPUT.split(",").map(hash_string).sum()
}

pub fn part2() -> usize {
    let mut map: [_; 256] = std::array::from_fn(|_| Vec::<(&str, u8)>::new());
    for operation in INPUT.split(",") {
        if let Some((lbl, num)) = operation.split_once('=') {
            let idx = hash_string(lbl);
            let num = num.parse::<u8>().unwrap();
            if let Some(lens) = map[idx].iter_mut().find(|(lbl2, _)| *lbl2 == lbl) {
                lens.1 = num;
            } else {
                map[idx].push((lbl, num));
            }
        } else if let Some(lbl) = operation.strip_suffix('-') {
            let idx = hash_string(lbl);
            if let Some(pos) = map[idx].iter().position(|(lbl2, _)| *lbl2 == lbl) {
                map[idx].remove(pos);
            }
        } else {
            #[cfg(debug_assertions)]
            unreachable!()
        };
    }
    let mut sum = 0;
    for (i, boxx) in map.iter().enumerate() {
        for (j, lens) in boxx.iter().enumerate() {
            sum += (i + 1) * (j + 1) * lens.1 as usize;
        }
    }
    sum
}
