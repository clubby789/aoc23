use std::fmt::Debug;

use rand::seq::SliceRandom;
use rustc_hash::{FxHashMap, FxHashSet};

/*
const INPUT: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
*/
const INPUT: &str = include_str!("inputs/25.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct NodeName(u32);

impl Debug for NodeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}""#, self.to_string())
    }
}

impl NodeName {
    pub fn from_str(s: &str) -> Self {
        let &[a, b, c] = s.as_bytes() else { panic!() };
        Self(u32::from_le_bytes([0, a, b, c]))
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.0.to_le_bytes()[1..].to_owned()).unwrap()
    }
}

fn get_rand_two(max: usize) -> (usize, usize) {
    let v1 = rand::random::<usize>() % max;
    let v2 = loop {
        let val = rand::random::<usize>() % max;
        if val != v1 {
            break val;
        }
    };
    (v1, v2)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Edge(NodeName, NodeName);

impl Edge {
    pub fn normalize(self) -> Self {
        let Self(from, to) = self;
        let mut edge = [from, to];
        edge.sort_unstable();
        Self(edge[0], edge[1])
    }
}

#[derive(Clone, Debug)]
struct Graph {
    // The usize counts the total number of vertices this 'supervertex' represents
    vertices: FxHashMap<NodeName, usize>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn from_input(input: &str) -> Self {
        let mut vertices = FxHashMap::default();
        let mut edges = Vec::new();
        for l in input.lines() {
            let (from, to) = l.split_once(": ").unwrap();
            let from = NodeName::from_str(from);
            vertices.insert(from, 1);
            for to in to.split_ascii_whitespace().map(NodeName::from_str) {
                vertices.insert(to, 1);
                edges.push(Edge(from, to).normalize());
            }
        }
        Self { vertices, edges }
    }

    /// Combine 'v2' into the node 'v1'
    /// This removes v2, and any edges to it will now link to v1
    pub fn contract(&mut self, v1: NodeName, v2: NodeName) {
        self.edges.remove(
            self.edges
                .iter()
                .position(|&Edge(e1, e2)| e1 == v1 && e2 == v2)
                .unwrap(),
        );
        let Some(count) = self.vertices.remove(&v2) else {
            unreachable!()
        };
        *self.vertices.get_mut(&v1).unwrap() += count;
        for i in (0..self.edges.len()).rev() {
            let edge = &mut self.edges[i];
            if edge.0 == v2 {
                edge.0 = v1;
            } else if edge.1 == v2 {
                edge.1 = v1;
            }
            if edge.0 == edge.1 {
                self.edges.remove(i);
            }
        }
    }
}

pub fn part1() -> usize {
    let mut rng = rand::thread_rng();
    let graph = Graph::from_input(INPUT);
    // Karger's algorithm: randomly combine vertices until we have 2
    // If we have 3 edges remaining (all equal), then we have successfully combined each graph 'half'
    // Multiply the number of merged vertices
    let squeezed = 'outer: loop {
        let mut current_graph = graph.clone();
        while current_graph.vertices.len() > 2 {
            let &Edge(from, to) = current_graph.edges.choose(&mut rng).unwrap();
            current_graph.contract(from, to);
        }
        if current_graph.edges.len() == 3 {
            break 'outer current_graph;
        }
    };
    squeezed.vertices.values().product()
}

pub fn part2() -> usize {
    0
}
