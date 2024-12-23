//! # LAN Party
//!
//! Brute force is still a thing.

use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use aoc_prelude::{lazy_static, Entry, HashMap, HashSet, Itertools};

const MAX_NODES: usize = 26 * 26;

lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
}

struct Graph {
    edges: [[bool; MAX_NODES]; MAX_NODES],
    nodes: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn default() -> Self {
        Self { edges: [[false; MAX_NODES]; MAX_NODES], nodes: HashMap::with_capacity(MAX_NODES) }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from][to] = true;
        self.edges[to][from] = true;

        self.nodes.entry(from).or_default().push(to);
        self.nodes.entry(to).or_default().push(from);
    }
}

fn solve() -> (usize, String) {
    let mut name_to_idx = HashMap::new();

    let mut graph = Graph::default();
    let mut tee_nodes = HashSet::new();

    include_str!("../../inputs/23.in").lines().for_each(|l| {
        let (fr, to) = l.split_once("-").unwrap();

        let (fr_idx, to_idx) = (idx(fr, &mut name_to_idx), idx(to, &mut name_to_idx));

        graph.add_edge(fr_idx, to_idx);

        if fr.starts_with("t") {
            tee_nodes.insert(fr_idx);
        }
        if to.starts_with("t") {
            tee_nodes.insert(to_idx);
        }
    });

    let p1 = count_triples(&graph, &tee_nodes);

    let idx_to_name = reverse(name_to_idx);

    let p2 = max_clique(&graph).iter().map(|idx| idx_to_name[idx]).sorted_unstable().join(",");

    (p1, p2)
}

fn idx<'a>(node: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
    match cache.entry(node) {
        Entry::Occupied(idx) => *idx.get(),
        Entry::Vacant(entry) => {
            let idx = COUNTER.fetch_add(1, Ordering::Relaxed);
            entry.insert(idx);
            idx
        }
    }
}

fn reverse<K, V: Eq + Hash>(h: HashMap<K, V>) -> HashMap<V, K> {
    h.into_iter().map(|(k, v)| (v, k)).collect()
}

fn count_triples(graph: &Graph, tee_nodes: &HashSet<usize>) -> usize {
    let mut seen = [false; MAX_NODES];
    let mut p1 = 0;

    for n1 in 0..MAX_NODES {
        if let Some(neighbours) = graph.nodes.get(&n1) {
            seen[n1] = true;

            for (i, &n2) in neighbours.iter().enumerate() {
                for &n3 in neighbours.iter().skip(i) {
                    if !seen[n2]
                        && !seen[n3]
                        && graph.edges[n2][n3]
                        && (tee_nodes.contains(&n1)
                            || tee_nodes.contains(&n2)
                            || tee_nodes.contains(&n3))
                    {
                        p1 += 1;
                    }
                }
            }
        }
    }
    p1
}

fn max_clique(graph: &Graph) -> Vec<usize> {
    let mut seen = [false; MAX_NODES];
    let mut clique = Vec::new();
    let mut max_clique = Vec::new();

    for (&cur, neighs) in &graph.nodes {
        if !seen[cur] {
            clique.clear();
            clique.push(cur);

            for &neigh in neighs {
                if clique.iter().all(|&c| graph.edges[c][neigh]) {
                    seen[neigh] = true;
                    clique.push(neigh);
                }
            }

            if clique.len() > max_clique.len() {
                max_clique.clone_from(&clique);
            }
        }
    }

    max_clique
}

aoc_2024::main! {
    solve()
}
