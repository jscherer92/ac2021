use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

// this can actually be found in the Rust docs: https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong");
    first_challenge(&contents);
    second_challenge(&contents);
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn first_challenge(s: &String) {
    let puzzle: Vec<Vec<usize>> = s.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect();
    let mut graph: Vec<Vec<Edge>> = Vec::new();

    for i in 0..puzzle.len() {
        for j in 0..puzzle[0].len() {
            let mut v: Vec<Edge> = Vec::new();
            if j != 0 {
                let left = (i * puzzle.len()) + j - 1;
                v.push(Edge { node: left, cost: puzzle[i][j-1]})
            }
            if i != 0 {
                let up = ((i * puzzle.len()) - puzzle.len()) + j;
                v.push(Edge { node: up, cost: puzzle[i-1][j]})
            }
            if i != puzzle.len()-1 {
                let down = ((i * puzzle.len()) + puzzle.len()) + j;
                v.push(Edge {node: down, cost: puzzle[i+1][j]})
            }
            if j != puzzle[0].len()-1 {
                let right = (i * puzzle.len()) + j + 1;
                v.push(Edge {node: right, cost: puzzle[i][j+1]})
            }
            graph.push(v);
        }
    }

    let ret = shortest_path(&graph, 0, (puzzle.len() * puzzle[0].len()) - 1);
    println!("{:?}", ret);
}

fn second_challenge(s: &String) {
    let puzzle: Vec<Vec<usize>> = s.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect();
    let puzzle_wide: Vec<Vec<usize>> = puzzle.iter().map(|line| {
        let mut v: Vec<usize> = vec![0; line.len() * 5];
        for i in 0..5 {
            for j in 0..line.len() {
                if i == 0 {
                    v[i + j] = line[i + j];
                } else {
                    v[i * line.len() + j] = v[(i * line.len() + j) - line.len()] + 1;
                    if v[i * line.len() + j] == 10 {
                        v[i * line.len() + j] = 1;
                    }
                }
            }
        }
        v
    }).collect();
    
    let mut full_puzzle: Vec<Vec<usize>> = vec![vec![0; puzzle_wide[0].len()]; puzzle_wide.len() * 5];
    for i in 0..puzzle_wide.len() {
        for j in 0..puzzle_wide[0].len() {
            full_puzzle[i][j] = puzzle_wide[i][j];
        }
    }
    for i in 0..4 {
        for j in 0..puzzle_wide.len() {
            for k in 0..puzzle_wide[0].len() {
                let curr_row = (i + 1) * 100 + j;
                let prev_row = i * 100 + j;
                let curr_col = k;
                full_puzzle[curr_row][curr_col] = full_puzzle[prev_row][curr_col] + 1;
                if full_puzzle[curr_row][curr_col] == 10 {
                    full_puzzle[curr_row][curr_col] = 1;
                }
            }
        }
    }

    let mut graph: Vec<Vec<Edge>> = Vec::new();

    for i in 0..full_puzzle.len() {
        for j in 0..full_puzzle[0].len() {
            let mut v: Vec<Edge> = Vec::new();
            if j != 0 {
                let left = (i * full_puzzle.len()) + j - 1;
                v.push(Edge { node: left, cost: full_puzzle[i][j-1]})
            }
            if i != 0 {
                let up = ((i * full_puzzle.len()) - full_puzzle.len()) + j;
                v.push(Edge { node: up, cost: full_puzzle[i-1][j]})
            }
            if i != full_puzzle.len()-1 {
                let down = ((i * full_puzzle.len()) + full_puzzle.len()) + j;
                v.push(Edge {node: down, cost: full_puzzle[i+1][j]})
            }
            if j != full_puzzle[0].len()-1 {
                let right = (i * full_puzzle.len()) + j + 1;
                v.push(Edge {node: right, cost: full_puzzle[i][j+1]})
            }
            graph.push(v);
        }
    }

    let ret = shortest_path(&graph, 0, (full_puzzle.len() * full_puzzle[0].len()) - 1);
    println!("{:?}", ret);
}