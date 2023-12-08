use seq_macro::seq;
use std::time::{Duration, Instant};
seq!(N in 1..=8 {
    #[allow(unused)]
    mod day~N;
});

fn timeit<F, U>(f: F) -> (Duration, U)
where
    F: Fn() -> U,
{
    // run a few times to get an estimate of timing
    let now = Instant::now();
    for _ in 0..32 {
        std::hint::black_box(f());
    }
    let initial_avg = now.elapsed() / 32;

    let measure_loops = (Duration::from_secs(5).as_nanos() / initial_avg.as_nanos()) as u32;

    let now = Instant::now();
    for _ in 0..measure_loops {
        std::hint::black_box(f());
    }
    let avg = now.elapsed() / measure_loops;
    let ret = std::hint::black_box(f());
    (avg, ret)
}

seq! {
    N in 1..=8 {
        static FUNCS: &[(fn() -> usize, fn() -> usize)] = &[
            #(
                (day~N::part1 as _, day~N::part2 as _),
            )*
        ];
    }
}

fn main() {
    let (f1, f2) = FUNCS.last().unwrap();
    if std::env::var("TIMEIT").is_ok() {
        let (t1, res) = timeit(f1);
        println!("Solved part 1 in {t1:?} - {res}");
        let (t2, res) = timeit(f2);
        println!("Solved part 2 in {t2:?} - {res}");
    } else {
        println!("Part 1 - {}", f1());
        println!("Part 2 - {}", f2());
    }
}
