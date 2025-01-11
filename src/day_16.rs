//https://adventofcode.com/2024/day/16

use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}, fs::File, io::{BufRead, BufReader}, vec};
use crate::misc_types::Point;

#[derive(Clone, Debug, PartialEq)]
struct Node {
    id: usize,
    position: Point<usize>,
    edges: Vec<Edge>,
    is_valid: bool,
}

impl Node {
    fn new(id: usize, x: usize, y: usize) -> Node {
        Node {
            id,
            position: Point::new(x, y),
            edges: vec![],
            is_valid: true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    fn new(from: usize, to: usize) -> Edge {
        Edge {
            from, 
            to,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node_id: usize,
    prev_node_id: Option<usize>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn calc_cost(from: Point<usize>, to: Point<usize>, current_direction: Point<i32>) -> usize {
    let p1: Point<i32> = from.into();
    let p2: Point<i32> = to.into();
    let diff = p2 - p1;

    //Calculate Manhattan cost, movement diagonally is not allowed or catered for
    let mut cost = (diff.x.abs() + diff.y.abs()) as usize;

    if current_direction != get_direction(p1, p2) {
        cost += 1000;
    }

    cost
}

fn get_direction(p1: Point<i32>, p2: Point<i32>) -> Point<i32> {
    let mut diff: Point<i32> = p2 - p1;

    diff = diff.clamp(-1, 1);

    diff
}

//Find nodes and edges and update map with their node id at their location
//All positive numbers in the map are nodes id's
fn find_nodes(map: &mut [Vec<i32>], nodes: &mut Vec<Node>) -> (usize, usize) {
    //Convert the map into a set of nodes and edges
    //First: Identify the nodes, these are:
    // - the start and end locations
    // - any point that is a junction or turn (not a corridor)
    // - any point that is a dead end

    let bounds: Point<usize> = Point::new(map[0].len(), map.len());

    let mut start: usize = 0;
    let mut end: usize = 0;

    for y in 1..bounds.y-1 {
        for x in 1..bounds.x-1 {
            match map[y][x] {
                //Empty space
                -1 => {
                    //If its not a corridor or completely enclosed...
                    if !((map[y][x-1] >= -1
                    && map[y][x+1] >= -1
                    && map[y-1][x] == -2
                    && map[y+1][x] == -2)
                    || (map[y-1][x] >= -1
                    && map[y+1][x] >= -1
                    && map[y][x-1] == -2
                    && map[y][x+1] == -2))
                    && (map[y][x-1] >= -1
                    || map[y][x+1] >= -1
                    || map[y-1][x] >= -1
                    || map[y+1][x] >= -1) {
                        let id = nodes.len();
                        map[y][x] = id as i32;
                        nodes.push(Node::new(id, x, y));
                    }
                },
                //Wall
                -2 => { /* Not a node */},
                //Start location
                -3 => {
                    start = nodes.len();
                    map[y][x] = start as i32;
                    nodes.push(Node::new(start, x, y));
                },
                //End location
                -4 => {
                    end = nodes.len();
                    map[y][x] = end as i32;
                    nodes.push(Node::new(end, x, y));
                },
                _ => { /* Ignored */ },
            };
        }
    }
    
    let directions: [Point<i32>; 4] = [
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];

    //A set of the unvisited nodes - any nodes left unvisited can be marked as invalid
    let mut not_visited: HashSet<usize> = nodes.iter().map(|n| n.id).collect();

    //Connect nodes together, starting with the start node
    let mut nodes_to_check: Vec<usize> = vec![start];
    while let Some(node_id) = nodes_to_check.pop() {
        if !not_visited.contains(&node_id) {
            continue;
        }

        //Look in each direction for another node
        for direction in directions {
            let mut curr_pos: Point<i32> = nodes[node_id].position.into();
            loop {
                let new_pos: Point<usize> = (curr_pos + direction).into();
                let cell_value = map[new_pos.y][new_pos.x];
                if cell_value >= 0 {
                    //We found another node, create an edge
                    nodes[node_id].edges.push(Edge::new(node_id, cell_value as usize));
                    nodes_to_check.push(cell_value as usize);
                    break;
                } else if cell_value == -2 {
                    //Hit a wall 
                    break;
                }

                curr_pos = new_pos.into();
            }
        }
        not_visited.remove(&node_id);
    }

    //Nodes with only 1 edge are dead ends, mark as invalid 
    //and remove any edges from other nodes that point to the invalid node
    // - repeat and until all nodes have at least 1 edge pointing to a valid node
    for node_id in 0..nodes.len() {
        if nodes[node_id].edges.len() == 1 && node_id != end {
            nodes[node_id].is_valid = false;
            let mut node_to_inspect = nodes[node_id].edges[0].to;
            let mut prev_node = node_id;
            loop {
                if nodes[node_to_inspect].edges.len() == 2 && node_to_inspect != start {
                    
                    nodes[node_to_inspect].is_valid = false;

                    if let Some(next_node) = nodes[node_to_inspect].edges
                        .iter()
                        .filter(|e|e.to != prev_node)
                        .map(|e| e.to)
                        .next() {
                        prev_node = node_to_inspect;
                        node_to_inspect = next_node;
                        
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    (start, end)
}

// Dijkstra's shortest path algorithm

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &[Node], start: usize, goal: usize, start_direction: Point<i32>) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, node_id: start, prev_node_id: None });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, node_id, prev_node_id }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if node_id == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[node_id] || !adj_list[node_id].is_valid { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[node_id].edges {
            let current_direction = if let Some(prev_node_id) = prev_node_id {
                get_direction(adj_list[prev_node_id].position.into(), adj_list[node_id].position.into())
            } else {
                start_direction
            };

            let edge_cost = calc_cost(adj_list[node_id].position, adj_list[edge.to].position, current_direction);
            let next = State { cost: cost + edge_cost, node_id: edge.to, prev_node_id: Some(node_id) };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.node_id] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.node_id] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 16 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day16.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line?;

        let row: Vec<i32> = line.chars().map(|c| match c {
            '#' => -2,  //Wall
            'S' => -3,  //Start location
            'E' => -4,  //End location
            _ => -1,    //Empty space
        }).collect();
        map.push(row);
    }

    let mut nodes: Vec<Node> = vec![];

    let (start, end) = find_nodes(&mut map, &mut nodes);

    //Start facing East
    let start_direction = Point::new(1, 0);

    if let Some(shortest_path_cost) = shortest_path(&nodes, start, end, start_direction) {
        Ok(shortest_path_cost as i64)
    } else {
        Ok(0)
    }
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 16 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day16.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<u8>> = vec![];

    for line in reader.lines() {
        let line = line?;

        let row: Vec<u8> = line.chars().map(|c| match c {
            '#' => 1,
            'S' => 2,
            'E' => 3,
            _ => 0,
        }).collect();
        map.push(row);
    }

    Ok(0)
}