use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("inputs/16.txt");

struct Grid<'a> {
    src: &'a [u8],
    // width including newline
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl<'a> Grid<'a> {
    pub fn new(src: &'a str) -> Self {
        let width = src.bytes().position(|b| b == b'\n').unwrap();
        Self {
            src: src.as_bytes(),
            width,
            height: src.len() / (width + 1),
        }
    }

    pub fn get(&self, (x, y): Pos) -> Option<&u8> {
        self.src.get(y * (self.width + 1) + x)
    }
}

type Pos = (usize, usize);

fn solve(grid: &Grid, mut pos: Pos, mut dir: Direction) -> usize {
    let mut points_todo: Vec<(Pos, Direction)> = Vec::new();
    let mut visited: FxHashSet<(Pos, Direction)> =
        FxHashSet::with_capacity_and_hasher(grid.src.len(), Default::default());
    fn next_pos((x, y): Pos, dir: Direction, grid: &Grid) -> Option<Pos> {
        Some(match dir {
            Direction::North if y > 0 => (x, y - 1),
            Direction::East if x + 1 < grid.width => (x + 1, y),
            Direction::South if y < grid.height => (x, y + 1),
            Direction::West if x > 0 => (x - 1, y),
            _ => return None,
        })
    }

    loop {
        if !visited.insert((pos, dir)) {
            if let Some((npos, ndir)) = points_todo.pop() {
                pos = npos;
                dir = ndir
            } else {
                break;
            }
        }
        let cell = grid.get(pos).unwrap();
        dir = match (cell, dir) {
            (b'.', _) => dir,
            (b'/', Direction::North) => Direction::East,
            (b'/', Direction::East) => Direction::North,
            (b'/', Direction::South) => Direction::West,
            (b'/', Direction::West) => Direction::South,
            (b'\\', Direction::North) => Direction::West,
            (b'\\', Direction::East) => Direction::South,
            (b'\\', Direction::South) => Direction::East,
            (b'\\', Direction::West) => Direction::North,
            (b'|', Direction::North | Direction::South) => dir,
            (b'-', Direction::East | Direction::West) => dir,
            (b'|', _) => {
                points_todo.push((pos, Direction::South));
                Direction::North
            }
            (b'-', _) => {
                points_todo.push((pos, Direction::East));
                Direction::West
            }
            _ => unreachable!("{pos:?}, '{}'", cell.escape_ascii()),
        };
        if let Some(npos) = next_pos(pos, dir, &grid) {
            pos = npos;
        } else if let Some((npos, ndir)) = points_todo.pop() {
            pos = npos;
            dir = ndir
        } else {
            break;
        }
    }
    let pos: FxHashSet<_> = visited.into_iter().map(|(pos, _)| pos).collect();
    pos.len()
}

pub fn part1() -> usize {
    solve(&Grid::new(INPUT), (0, 0), Direction::East)
}

pub fn part2() -> usize {
    let g = Grid::new(INPUT);
    let mut largest = 0;
    for x in 0..g.width {
        largest = largest.max(solve(&g, (x, 0), Direction::South)).max(solve(
            &g,
            (x, g.height - 1),
            Direction::North,
        ));
    }
    for y in 0..g.height {
        largest = largest.max(solve(&g, (0, y), Direction::East)).max(solve(
            &g,
            (g.width - 1, y),
            Direction::West,
        ));
    }
    largest
}
