use rustc_hash::FxHashMap;
use std::{
    cell::{Cell, OnceCell, RefCell},
    collections::VecDeque,
};

const INPUT: &str = include_str!("inputs/20.txt");

type ModuleKey = u16;

struct Circuit {
    map: Box<[Option<Module>; u16::MAX as usize + 1]>,
    signals: RefCell<VecDeque<(ModuleKey, ModuleKey, bool)>>,
}

impl Circuit {
    pub fn get(&self, name: ModuleKey) -> Option<&Module> {
        self.map[name as usize].as_ref()
    }

    pub fn send_signal(&self, from: ModuleKey, to: ModuleKey, signal: bool) {
        self.signals.borrow_mut().push_back((from, to, signal));
    }

    pub fn process<F: FnMut(ModuleKey, ModuleKey, bool)>(&self, mut f: F) {
        let mut longest = 0;
        loop {
            let Some((from, to, signal)) = self.signals.borrow_mut().pop_front() else {
                break;
            };
            longest = longest.max(self.signals.borrow().len());
            f(from, to, signal);
            match self.get(to).map(|m| (m, &m.kind)) {
                None => {
                    debug_assert_eq!(to, RX);
                }
                Some((_, ModuleKind::FlipFlop { .. })) if signal => {}
                Some((to, ModuleKind::FlipFlop { ref on })) => {
                    if on.get() {
                        on.set(false);
                        self.signals
                            .borrow_mut()
                            .extend(to.targets.iter().map(|t: &u16| (to.name, *t, false)));
                    } else {
                        on.set(true);
                        self.signals
                            .borrow_mut()
                            .extend(to.targets.iter().map(|t| (to.name, *t, true)));
                    }
                }
                Some((to, ModuleKind::Broadcaster)) => {
                    self.signals
                        .borrow_mut()
                        .extend(to.targets.iter().map(|t| (to.name, *t, false)));
                }
                Some((to, ModuleKind::Conjunction { ref memory })) => {
                    memory
                        .borrow()
                        .iter()
                        .find(|(k, _)| *k == from)
                        .unwrap()
                        .1
                        .set(signal);
                    if memory.borrow().iter().all(|v| v.1.get()) {
                        self.signals
                            .borrow_mut()
                            .extend(to.targets.iter().map(|t| (to.name, *t, false)));
                    } else {
                        self.signals
                            .borrow_mut()
                            .extend(to.targets.iter().map(|t| (to.name, *t, true)));
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, &Module)> {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(i, module)| module.as_ref().map(|m| (i, m)))
            .take(100)
            .map(|(i, md)| (i as u16, md))
    }
}

#[derive(Debug)]
struct Module {
    kind: ModuleKind,
    name: ModuleKey,
    targets: Vec<ModuleKey>,
}

#[derive(Debug)]
enum ModuleKind {
    Broadcaster,
    FlipFlop {
        on: Cell<bool>,
    },
    // Map of input modules to their most recent signal
    Conjunction {
        memory: RefCell<Vec<(ModuleKey, Cell<bool>)>>,
    },
}

const fn name_to_key(name: &str) -> ModuleKey {
    match name.as_bytes() {
        &[hi, lo] => u16::from_be_bytes([hi, lo]),

        #[cfg(debug_assertions)]
        b"broadcaster" => BROADCASTER,
        #[cfg(debug_assertions)]
        _ => unreachable!(),

        #[cfg(not(debug_assertions))]
        _ => BROADCASTER,
    }
}

fn parse_line(input: &str) -> Module {
    let (name, stuff) = input.split_once(" -> ").unwrap();
    let targets = stuff
        .split(", ")
        .filter(|s| !s.is_empty())
        .map(name_to_key)
        .collect();

    let (kind, name) = if let ("%", name) = name.split_at(1) {
        (
            ModuleKind::FlipFlop {
                on: Cell::new(false),
            },
            name_to_key(name),
        )
    } else if let ("&", name) = name.split_at(1) {
        (
            ModuleKind::Conjunction {
                memory: RefCell::new(Vec::with_capacity(4)),
            },
            name_to_key(name),
        )
    } else {
        debug_assert_eq!(name, "broadcaster");
        (ModuleKind::Broadcaster, BROADCASTER)
    };
    Module {
        kind,
        name,
        targets,
    }
}

const RX: ModuleKey = name_to_key("rx");

const EMPTY: Option<Module> = None;

fn make_map(input: &str) -> Circuit {
    let mut map = Box::new([EMPTY; u16::MAX as usize + 1]);
    for line in input.lines() {
        let module = parse_line(line);
        let name = module.name;
        map[name as usize] = Some(module);
    }

    for (name, module) in map
        .iter()
        .enumerate()
        .filter_map(|(i, module)| module.as_ref().map(|m| (i, m)))
        .take(100)
    {
        for tgt in module.targets.iter() {
            if let Some(Some(tgt)) = map.get(*tgt as usize) {
                if let ModuleKind::Conjunction { memory } = &tgt.kind {
                    memory
                        .borrow_mut()
                        .push((name as ModuleKey, Cell::new(false)));
                }
            }
        }
    }
    Circuit {
        map,
        signals: RefCell::new(VecDeque::with_capacity(32)),
    }
}

const BTN: ModuleKey = ModuleKey::MAX - 1;
const BROADCASTER: ModuleKey = ModuleKey::MAX;

pub fn part1() -> usize {
    let circuit = make_map(INPUT);
    let mut lo_count = 0;
    let mut hi_count = 0;
    let broadcast = circuit.get(BROADCASTER);
    for _ in 0..1000 {
        // lo_count += 1;
        circuit.send_signal(BTN, BROADCASTER, false);
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
        .iter()
        .map(|(_, v)| v)
        .find(|module| module.targets.contains(&RX))
        .unwrap();
    let ModuleKind::Conjunction { memory } = &before_rx.kind else {
        unreachable!()
    };

    let before_before_rx: FxHashMap<_, _> = memory
        .borrow()
        .iter()
        .map(|k| (k.0, OnceCell::new()))
        .collect();

    let mut cycle_count = 0;
    let mut i = 0;

    while cycle_count < before_before_rx.len() {
        i += 1;
        circuit.send_signal(BTN, BROADCASTER, false);
        circuit.process(|from, to, signal| {
            if signal && to == before_rx.name {
                let entry = &before_before_rx[&from];
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
