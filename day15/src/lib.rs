//! Library module with all the logic
use petgraph::prelude::*;
use petgraph::algo::dijkstra;

use std::collections::HashMap;

type UndirGraph = Graph<(usize, usize), usize>;
type NodeDict = HashMap<(usize, usize), NodeIndex>;

/// Solve Advent of Code day 15 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 15.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    // Build a grid of data
    let grid: HashMap<(usize, usize), usize> = build_grid(data);
    let x_max = data.trim().split('\n').count();
    let y_max = data.trim().split('\n').next().unwrap().len();
    // Build a graph
    let (graph, dic) = build_graph(&grid);
    let start: NodeIndex = *dic.get(&(0, 0)).unwrap();
    let end: NodeIndex = *dic.get(&(y_max-1, x_max-1)).unwrap();
    let res = dijkstra(&graph,
                       start, // Start
                       Some(end), // End
            |e| *e.weight());
    *res.get(&end).unwrap()
}

fn build_grid(data: &str) -> HashMap<(usize, usize), usize> {
    data.trim().split('\n')
        .enumerate()
        .flat_map(|(x, line)| line.chars()
             .enumerate()
             .map(move |(y, c)|
                  ((y, x), String::from(c).parse::<usize>().unwrap())
            ))
        .collect::<HashMap<(usize, usize), usize>>()
}

fn build_graph(data: &HashMap<(usize, usize), usize>) -> (UndirGraph, NodeDict) {
    // Build this graph
    let mut res: UndirGraph = Graph::new();
    // Build the dict of nodes
    let nodes = data.keys()
        .map(|&(y, x)| ((y, x), res.add_node((y, x))))
        .collect::<NodeDict>();
    // For all coordinates, create an edge from its possible upstream
    let extendable_edges = data.iter()
        .flat_map(|(&(y, x), &v)| {
            let mut r = Vec::new();
            let me: NodeIndex = *nodes.get(&(y, x)).unwrap();
            // Possible up?
            if y > 0 {
                let topnode = *nodes.get(&(y-1, x)).unwrap();
                let topvalue = *data.get(&(y-1, x)).unwrap();
                r.push((topnode, me, v));
                r.push((me, topnode, topvalue));
            }
            if x > 0 {
                let botnode = *nodes.get(&(y, x-1)).unwrap();
                let botvalue = *data.get(&(y, x-1)).unwrap();
                r.push((botnode, me, v));
                r.push((me, botnode, botvalue));
            }
            r
        })
        .collect::<Vec<(NodeIndex, NodeIndex, usize)>>();
    res.extend_with_edges(extendable_edges);
    (res, nodes)
}

/// Solve Advent of Code day 15 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 15.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    // Build a grid of data
    let grid: HashMap<(usize, usize), usize> = build_grid(data);
    let x_max = data.trim().split('\n').count();
    let y_max = data.trim().split('\n').next().unwrap().len();
    // Build a graph
    let (graph, dic) = build_large_graph(&grid, y_max, x_max);
    let start: NodeIndex = *dic.get(&(0, 0)).unwrap();
    let end: NodeIndex = *dic.get(&(5*y_max-1, 5*x_max-1)).unwrap();
    let res = dijkstra(&graph,
                       start, // Start
                       Some(end), // End
            |e| *e.weight());
    *res.get(&end).unwrap()
}

fn enlarge_risk_value(y: usize, x: usize, x_max: usize, y_max: usize, val: usize) -> usize {
    (val - 1 + (x/x_max) + (y/y_max))%9+1
}

fn build_large_graph(data: &HashMap<(usize, usize), usize>, y_size: usize, x_size: usize) -> (UndirGraph, NodeDict) {
    // Build this graph
    let mut res: UndirGraph = Graph::new();
    // Build the dict of nodes
    let nodes = data.keys()
        .flat_map(|&(y, x)|
              (0..5).flat_map(|dy| {
                  let my_y = dy*y_size+y;
                vec![
                    ((my_y, x), res.add_node((dy*y_size+y, x))),
                    ((my_y, x_size+x), res.add_node((dy*y_size+y, x_size+x))),
                    ((my_y, 2*x_size+x), res.add_node((my_y, 2*x_size+x))),
                    ((my_y, 3*x_size+x), res.add_node((my_y, 3*x_size+x))),
                    ((my_y, 4*x_size+x), res.add_node((my_y, 4*x_size+x)))
                ]
            })
            .collect::<Vec<((usize, usize), NodeIndex)>>()
        )
        .collect::<NodeDict>();
    // For all coordinates, create an edge from its possible upstream
    let extendable_edges = nodes.iter()
        .flat_map(|(&(y, x), &me)| {
            let mut r = Vec::new();
            let v = enlarge_risk_value(y, x, y_size, x_size, *data.get(&(y%y_size, x%x_size)).unwrap());
            // Possible up?
            if y > 0 {
                let topnode = *nodes.get(&(y-1, x)).unwrap();
                let topvalue = enlarge_risk_value(y-1, x, y_size, x_size, *data.get(&((y-1)%y_size, x%x_size)).unwrap());
                r.push((topnode, me, v));
                r.push((me, topnode, topvalue));
            }
            if x > 0 {
                let botnode = *nodes.get(&(y, x-1)).unwrap();
                let botvalue = enlarge_risk_value(y, x-1, y_size, x_size, *data.get(&(y%y_size, (x-1)%x_size)).unwrap());
                r.push((botnode, me, v));
                r.push((me, botnode, botvalue));
            }
            r
        })
        .collect::<Vec<(NodeIndex, NodeIndex, usize)>>();
    res.extend_with_edges(extendable_edges);
    (res, nodes)
}

