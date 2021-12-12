//! Library module with all the logic

use petgraph::prelude::*;
use std::collections::{HashMap, VecDeque, HashSet};

/// Solve Advent of Code day 12 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 12.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let mut graph = Graph::<&str, (), Undirected>::new_undirected();
    let edges = data.trim().split('\n')
        .map(|line| {
            let sl = line.split('-').collect::<Vec<&str>>();
            (*sl.get(0).unwrap(), *sl.get(1).unwrap())
        })
        .collect::<Vec<(&str, &str)>>();
    let mut vertice_names = edges.iter()
        .flat_map(|x| vec![x.0, x.1])
        .collect::<Vec<&str>>();
    vertice_names.dedup();
   let vertices = vertice_names.iter()
        .map(|&v| (v, graph.add_node(v)))
        .collect::<HashMap<&str, NodeIndex>>();
   let small_vertices = vertice_names.iter()
        .filter(|&x| x.chars().any(char::is_lowercase))
        .copied()
        .map(|x| vertices.get(x).unwrap())
        .collect::<HashSet<&NodeIndex>>();
   edges.iter()
        .map(|&(s, e)| (vertices.get(s).unwrap(), vertices.get(e).unwrap()))
        .for_each(|s| {graph.add_edge(*s.0, *s.1, ());});

    // Compute the trivial graph thingy
    messed_up_dfs(&graph,
        *vertices.get("start").unwrap(),
        *vertices.get("end").unwrap(),
        &small_vertices)
}

fn messed_up_dfs(graph: &Graph<&str, (), Undirected, u32>,
                 a: NodeIndex, b: NodeIndex, small: &HashSet<&NodeIndex>) -> usize {
    let mut paths: VecDeque<Vec<NodeIndex>> = VecDeque::new();
    let mut finished: usize = 0;
    paths.push_front(vec![a]);
    while !paths.is_empty() {
        // Pop a path
        let current_study = paths.pop_front().unwrap();
        let back = *current_study.last().unwrap();
        // If it's the end, stop
        if back == b {
            finished += 1;
            continue;
        }
        // Find its neighbors
        for neigh in graph.neighbors(back) {
            if small.contains(&neigh) && current_study.contains(&neigh) {
                continue;
            }
            // Push new paths to the back
            let mut v_new: Vec<NodeIndex> = current_study.clone();
            v_new.push(neigh);
            paths.push_back(v_new);
        }
    }
    finished
}

fn even_more_messed_up_dfs(graph: &Graph<&str, (), Undirected, u32>,
                           a: NodeIndex, b: NodeIndex,
                           small: &HashSet<&NodeIndex>) -> usize {
    let mut finished: usize = 0;
    let mut paths: VecDeque<(bool, Vec<NodeIndex>)> = VecDeque::new();
    paths.push_front((false, vec![a]));
    while !paths.is_empty() {
        // Pop a path
        let (small_visited_twice, current_study) = paths.pop_front().unwrap();
        let back = *current_study.last().unwrap();
        // If it's the end, stop
        if back == b {
            finished += 1;
            continue;
        }
        // Find its neighbors
        for neigh in graph.neighbors(back) {
            let mut duped: bool = false;
            if small.contains(&neigh) && current_study.contains(&neigh) {
                if a == neigh || small_visited_twice {
                    continue;
                } else if current_study.iter()
                    .filter(|&x| *x == neigh).count() == 1 {
                    duped = true;
                }
            }
            // Push new paths to the back
            let mut v_new: Vec<NodeIndex> = current_study.clone();
            v_new.push(neigh);
            paths.push_back((small_visited_twice || duped, v_new));
        }
    }
    finished
}

/// Solve Advent of Code day 12 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 12.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let mut graph = Graph::<&str, (), Undirected>::new_undirected();
    let edges = data.trim().split('\n')
        .map(|line| {
            let sl = line.split('-').collect::<Vec<&str>>();
            (*sl.get(0).unwrap(), *sl.get(1).unwrap())
        })
        .collect::<Vec<(&str, &str)>>();
    let mut vertice_names = edges.iter()
        .flat_map(|x| vec![x.0, x.1])
        .collect::<Vec<&str>>();
    vertice_names.dedup();
   let vertices = vertice_names.iter()
        .map(|&v| (v, graph.add_node(v)))
        .collect::<HashMap<&str, NodeIndex>>();
   let small_vertices = vertice_names.iter()
        .filter(|&x| x.chars().any(char::is_lowercase))
        .copied()
        .map(|x| vertices.get(x).unwrap())
        .collect::<HashSet<&NodeIndex>>();
   edges.iter()
        .map(|&(s, e)| (vertices.get(s).unwrap(), vertices.get(e).unwrap()))
        .for_each(|s| {graph.add_edge(*s.0, *s.1, ());});

    // Compute the trivial graph thingy
    even_more_messed_up_dfs(&graph,
        *vertices.get("start").unwrap(),
        *vertices.get("end").unwrap(),
        &small_vertices)
}

