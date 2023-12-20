use rustc_hash::FxHashMap;
use std::{
    cell::{Cell, OnceCell, RefCell},
    collections::VecDeque,
};

const INPUT: &str = include_str!("inputs/20.txt");

struct Circuit<'a> {
    map: FxHashMap<&'a str, Module<'a>>,
    signals: RefCell<VecDeque<(&'a str, &'a str, bool)>>,
}

impl<'a> Circuit<'a> {
    pub fn get(&self, name: &'a str) -> &Module<'a> {
        self.map.get(&name).unwrap()
    }

    pub fn send_signal(&self, from: &'a str, to: &'a str, signal: bool) {
        self.signals.borrow_mut().push_back((from, to, signal));
    }

    pub fn process<F: FnMut(&'a str, &'a str, bool)>(&self, mut f: F) {
        loop {
            let Some((from, to, signal)) = self.signals.borrow_mut().pop_front() else {
                break;
            };
            f(from, to, signal);
            let mut pending = VecDeque::with_capacity(1);
            //println!("handling {from} -> {to} ({signal})");
            let to = self.get(to);
            match &to.kind {
                ModuleKind::FlipFlop { .. } if signal => {}
                ModuleKind::FlipFlop { ref on } => {
                    if on.get() {
                        on.set(false);
                        pending.extend(to.targets.iter().map(|t| (to.name, *t, false)));
                    } else {
                        on.set(true);
                        pending.extend(to.targets.iter().map(|t| (to.name, *t, true)));
                    }
                }
                ModuleKind::Broadcaster => {
                    pending.extend(to.targets.iter().map(|t| (to.name, *t, false)));
                }
                ModuleKind::Conjunction { ref memory } => {
                    memory.borrow()[from].set(signal);
                    if memory.borrow().values().all(|v| v.get()) {
                        pending.extend(to.targets.iter().map(|t| (to.name, *t, false)));
                    } else {
                        pending.extend(to.targets.iter().map(|t| (to.name, *t, true)));
                    }
                }
                ModuleKind::None { ref pressed } => {
                    if !signal {
                        pressed.set(true);
                    }
                }
            }
            self.signals.borrow_mut().append(&mut pending);
        }
    }
}

#[derive(Debug)]
struct Module<'a> {
    kind: ModuleKind<'a>,
    name: &'a str,
    targets: Vec<&'a str>,
}

#[derive(Debug)]
enum ModuleKind<'a> {
    Broadcaster,
    FlipFlop {
        on: Cell<bool>,
    },
    // Map of input modules to their most recent signal
    Conjunction {
        memory: RefCell<FxHashMap<&'a str, Cell<bool>>>,
    },
    None {
        pressed: Cell<bool>,
    },
}

fn parse_line(input: &str) -> Module<'_> {
    let (name, stuff) = input.split_once(" -> ").unwrap();
    let targets = stuff.split(", ").filter(|s| !s.is_empty()).collect();

    let (kind, name) = if let ("%", name) = name.split_at(1) {
        (
            ModuleKind::FlipFlop {
                on: Cell::new(false),
            },
            name,
        )
    } else if let ("&", name) = name.split_at(1) {
        (
            ModuleKind::Conjunction {
                memory: RefCell::new(FxHashMap::default()),
            },
            name,
        )
    } else {
        debug_assert_eq!(name, "broadcaster");
        (ModuleKind::Broadcaster, "broadcaster")
    };
    Module {
        kind,
        name,
        targets,
    }
}

fn make_map(input: &str) -> Circuit<'_> {
    let mut map: FxHashMap<_, _> = input
        .lines()
        .map(parse_line)
        .map(|module| (module.name, module))
        .chain(std::iter::once((
            "rx",
            Module {
                kind: ModuleKind::None {
                    pressed: Cell::new(false),
                },
                name: "rx",
                targets: vec![],
            },
        )))
        .collect();
    for (name, module) in map.iter() {
        for tgt in module.targets.iter() {
            if let Some(tgt) = map.get(tgt) {
                if let ModuleKind::Conjunction { memory } = &tgt.kind {
                    memory.borrow_mut().insert(name, Cell::new(false));
                }
            }
        }
    }
    Circuit {
        map,
        signals: RefCell::new(VecDeque::with_capacity(8)),
    }
}

pub fn part1() -> usize {
    let circuit = make_map(INPUT);
    let mut lo_count = 0;
    let mut hi_count = 0;
    let broadcast = circuit.get("broadcaster");
    for _ in 0..1000 {
        // lo_count += 1;
        circuit.send_signal("btn", "broadcaster", false);
        circuit.process(|_, _, signal| {
            if signal {
                hi_count += 1;
            } else {
                lo_count += 1;
            }
        });
    }

    lo_count * hi_count
}

fn lcm(a: usize, b: usize) -> usize {
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

pub fn part2() -> usize {
    let circuit = make_map(INPUT);
    let before_rx = circuit
        .map
        .values()
        .find(|module| module.targets.contains(&"rx"))
        .unwrap();
    let ModuleKind::Conjunction { memory } = &before_rx.kind else {
        unreachable!()
    };

    let before_before_rx: FxHashMap<_, _> = memory
        .borrow()
        .keys()
        .map(|k| (*k, OnceCell::new()))
        .collect();

    let mut cycle_count = 0;
    let mut i = 0;

    while cycle_count < before_before_rx.len() {
        i += 1;
        circuit.send_signal("btn", "broadcaster", false);
        circuit.process(|from, to, signal| {
            if signal && to == before_rx.name {
                let entry = &before_before_rx[from];
                if entry.set(i).is_ok() {
                    cycle_count += 1;
                }
            }
        });
    }
    before_before_rx.iter().fold(1, |acc, (key, val)| {
        let i = *val.get().unwrap();
        lcm(acc, i)
    })
}
