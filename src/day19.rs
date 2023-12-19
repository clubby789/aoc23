use rustc_hash::FxHashMap;
use std::ops::Range;

const INPUT: &str = include_str!("inputs/19.txt");
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    pub fn sum(&self) -> usize {
        self.x as usize + self.m as usize + self.a as usize + self.s as usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum WorkflowDest<'a> {
    Reject,
    Accept,
    Workflow(&'a str),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cmp {
    Lesser,
    Greater,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Prop {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum WorkflowStep<'a> {
    Part {
        prop: Prop,
        cmp: Cmp,
        value: u16,
        dest: WorkflowDest<'a>,
    },
    Final(WorkflowDest<'a>),
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<WorkflowStep<'a>>,
}

fn parse_dest(input: &str) -> WorkflowDest<'_> {
    match input {
        "A" => WorkflowDest::Accept,
        "R" => WorkflowDest::Reject,
        _ => WorkflowDest::Workflow(input),
    }
}

fn parse_workflow(input: &str) -> (&str, Workflow<'_>) {
    let (name, rest) = input.split_once('{').unwrap();
    let rest = rest.strip_suffix('}').unwrap();
    let rules = rest
        .split(",")
        .map(|step| {
            if let Some((check, dest)) = step.split_once(':') {
                let [prop, cmp, value @ ..] = check.as_bytes() else {
                    unreachable!()
                };
                let prop = match prop {
                    b'x' => Prop::X,
                    b'm' => Prop::M,
                    b'a' => Prop::A,
                    b's' => Prop::S,
                    _ => unreachable!(),
                };
                let cmp = if *cmp == b'<' {
                    Cmp::Lesser
                } else {
                    Cmp::Greater
                };
                debug_assert!(value.iter().all(|c| c.is_ascii_digit()));
                let value = value
                    .iter()
                    .fold(0u16, |acc, x| (acc * 10) + (*x & 0b1111) as u16);
                let dest = parse_dest(dest);
                WorkflowStep::Part {
                    prop,
                    cmp,
                    value,
                    dest,
                }
            } else {
                WorkflowStep::Final(parse_dest(step))
            }
        })
        .collect();
    (name, Workflow { rules })
}

fn parse_part(input: &str) -> Part {
    let line = input.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
    let mut iter = line.split(",").map(|c| c.split_once('=').unwrap().1);
    Part {
        x: iter.next().unwrap().parse().unwrap(),
        m: iter.next().unwrap().parse().unwrap(),
        a: iter.next().unwrap().parse().unwrap(),
        s: iter.next().unwrap().parse().unwrap(),
    }
}

fn parse(input: &str) -> (Vec<Part>, FxHashMap<&str, Workflow<'_>>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(parse_workflow).collect();
    let parts = parts.lines().map(parse_part).collect();
    (parts, workflows)
}

// return `true` if the check passes
fn run_step(part: Part, prop: Prop, cmp: Cmp, value: u16) -> bool {
    let part_value = match prop {
        Prop::X => part.x,
        Prop::M => part.m,
        Prop::A => part.a,
        Prop::S => part.s,
    };
    match cmp {
        Cmp::Lesser => part_value < value,
        Cmp::Greater => part_value > value,
    }
}

pub fn part1() -> usize {
    let (parts, workflows) = parse(INPUT);
    parts
        .into_iter()
        .map(|part| {
            let mut cur = WorkflowDest::Workflow("in");
            while let WorkflowDest::Workflow(name) = cur {
                for step in &workflows[&name].rules {
                    match step {
                        &WorkflowStep::Part {
                            prop,
                            cmp,
                            value,
                            dest,
                        } => {
                            if run_step(part, prop, cmp, value) {
                                cur = dest;
                                break;
                            }
                        }
                        &WorkflowStep::Final(dest) => {
                            cur = dest;
                            break;
                        }
                    }
                }
            }
            if matches!(cur, WorkflowDest::Accept) {
                part.sum()
            } else {
                0
            }
        })
        .sum()
}

#[derive(Clone, Debug)]
struct SymbolicPart {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>,
}

impl SymbolicPart {
    pub fn count(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

// first range is the matching range, second is the non-matching one
fn split_range(range: &Range<u16>, cmp: Cmp, value: u16) -> Option<(Range<u16>, Range<u16>)> {
    match cmp {
        // (50..100), < 10
        // will never match, return empty ranges
        Cmp::Lesser if value <= range.start => Some((0..0, 0..0)),
        // (1..100),  < 5
        // may match, split range into matching and not
        Cmp::Lesser if value < range.end => Some((range.start..value, value..range.end)),

        // (1..100), < 100
        // will always match, no need to split
        Cmp::Lesser if value >= range.end => None,

        // (50..100), > 100
        // will never match, return empty ranges
        Cmp::Greater if value >= range.end => Some((0..0, 0..0)),
        // (1..100),  > 5
        // may match, split range into matching and not
        Cmp::Greater if value < range.end => Some((value + 1..range.end, range.start..value + 1)),
        // (2..100), > 1
        // will always match, no need to split
        Cmp::Lesser if value >= range.end => None,
        _ => unreachable!("{cmp:?}, {range:?}, {value}"),
    }
}

pub fn part2() -> usize {
    let workflows: FxHashMap<_, _> = INPUT
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(parse_workflow)
        .collect();
    // stack of a workflow, the current step we're on, and the symbolic part
    // before being constrained by this step
    let mut stack = vec![(
        &workflows["in"],
        0,
        SymbolicPart {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
    )];
    let mut sum = 0;
    'stack: while let Some((mut wf, mut step, mut part)) = stack.pop() {
        while let WorkflowStep::Final(dest) = wf.rules[step] {
            match dest {
                WorkflowDest::Reject => continue 'stack,
                WorkflowDest::Accept => {
                    sum += part.count();
                    continue 'stack;
                }
                WorkflowDest::Workflow(dest) => {
                    wf = &workflows[dest];
                    step = 0;
                }
            }
        }
        while let WorkflowStep::Part {
            prop,
            cmp,
            value,
            dest,
        } = wf.rules[step]
        {
            let mut part2 = part.clone();
            let field = match prop {
                Prop::X => &mut part.x,
                Prop::M => &mut part.m,
                Prop::A => &mut part.a,
                Prop::S => &mut part.s,
            };
            if let Some((r1, r2)) = split_range(field, cmp, value) {
                *match prop {
                    Prop::X => &mut part2.x,
                    Prop::M => &mut part2.m,
                    Prop::A => &mut part2.a,
                    Prop::S => &mut part2.s,
                } = r1;
                match dest {
                    WorkflowDest::Reject => (),
                    WorkflowDest::Accept => sum += part2.count(),
                    WorkflowDest::Workflow(dest) => stack.push((&workflows[&dest], 0, part2)),
                }
                *field = r2;
            }
            step += 1;
        }
        let WorkflowStep::Final(dest) = wf.rules[step] else {
            unreachable!()
        };
        match dest {
            WorkflowDest::Reject => continue 'stack,
            WorkflowDest::Accept => {
                sum += part.count();
                continue 'stack;
            }
            WorkflowDest::Workflow(dest) => {
                stack.push((&workflows[dest], 0, part));
            }
        }
    }
    sum
}
