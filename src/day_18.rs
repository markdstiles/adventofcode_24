//https://adventofcode.com/2024/day/18

use std::{collections::{HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

pub fn do_part1() -> anyhow::Result<i32> {
    println!("Day 18 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day18.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let max_bytes = 1024;
    let mem_width = 71;
    let mem_height = 71;

    //Track which cells are corrupted
    let mut corrupted_memory_space: HashSet<usize> = HashSet::new();

    //Drop our imaginary 1024 bytes into the two-dimensional memory space and mark those cells as corrupt
    for (byte_count, line) in reader.lines().enumerate() {
        let line = line?;

        if byte_count == max_bytes { break }

        if let Some((x, y)) = line.split_once(",")
            .map(|(x,y)| (
                x.parse::<usize>().unwrap(), 
                y.parse::<usize>().unwrap())
            ) { corrupted_memory_space.insert((y * mem_width) + x); };
    }
    
    //Perform Breadth First Search (BFS) to find a path from 0,0 to 70,70
    //We'll convert the x,y into a cell_id/index = (y + width) + x
    //using it to check if we can move into the next cell, if its not corrupted
    let num_cells = mem_width * mem_height;
    let start_cell = 0;
    let goal_cell = num_cells-1;
    let mut visited: Vec<bool> = vec![false; num_cells];
    let mut route: Vec<i32> = vec![-1; num_cells];
    let mut goal_reached = false;
    let mut queue: VecDeque<Edge> = VecDeque::new();
    //The directions we can move
    let directions: &[(i32,i32)] = &[
        (0, -1),    //North
        (1, 0),     //East
        (0, 1),     //South
        (-1, 0),    //West
    ];

    //Add the start cell
    queue.push_back(Edge::new(start_cell, start_cell));

    while let Some(next) = queue.pop_front() {
        //Mark the parent of the cell
        route[next.to] = next.from as i32;

        //If the goal is reached, stop
        if next.to == goal_cell {
            goal_reached = true;
            break;
        }

        //For each potential direction, add an edge to the queue
        //as long as we haven't been there before and its within
        //bounds
        for dir in directions {
            //Edges from next.to to visit
            let x = next.to % mem_width;
            let y = (next.to - x) / mem_height;
            let (to_x, to_y) = (x as i32 + dir.0, y as i32 + dir.1);
            //Bounds check
            if to_x >= 0 && to_x < mem_width as i32
            && to_y >= 0 && to_y < mem_height as i32 {
                let to_cell = (to_y as usize * mem_height) + to_x as usize;
                //Check whether we've visited the cell before or if it is corrupted
                if !visited[to_cell] && !corrupted_memory_space.contains(&to_cell) {
                    queue.push_back(Edge::new(next.to, to_cell));
                    //Important: cell is marked as visited before its processed 
                    //to reduce number of edges being processed
                    visited[to_cell] = true;
                }
            }
        }
    }

    if goal_reached {
        let mut steps = 0;
        let mut cell_id = goal_cell;
        while cell_id != start_cell {
            cell_id = route[cell_id] as usize;
            steps += 1;
        }

        Ok(steps)
    } else {
        anyhow::bail!("Failed to find goal!")
    }
}

pub fn do_part2() -> anyhow::Result<String> {
    println!("Day 18 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day18.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let max_bytes = 1024;
    let mem_width = 71;
    let mem_height = 71;
    let num_cells = mem_width * mem_height;

    //Track which cells are corrupted (didn't need to be a HashSet)
    let mut corrupted_memory_space: Vec<bool> = vec![false; num_cells];
    //These will be the cell_ids of the remaining bytes to drop
    let mut bytes_to_drop: VecDeque<usize> = VecDeque::new();

    for (byte_count, line) in reader.lines().enumerate() {
        let line = line?;

        if let Some((x, y)) = line.split_once(",")
            .map(|(x,y)| (
                x.parse::<usize>().unwrap(), 
                y.parse::<usize>().unwrap())
            ) { 
                //We know we can path through 1024 bytes so add those to the corrupted memory spaces
                if byte_count < max_bytes {
                    corrupted_memory_space[(y * mem_width) + x] = true; 
                } else {
                    //Leave the remaining to be dropped
                    bytes_to_drop.push_back((y * mem_width) + x);
                }
            };
    }
    
    //For Part 2 we'll iterate through the remaining blocks until we can't find a path to the exit

    //Perform Breadth First Search (BFS) to find a path from 0,0 to 70,70
    //We'll convert the x,y into a cell_id/index = (y + width) + x
    //using it to check if we can move into the next cell, if its not corrupted
    let start_cell = 0;
    let goal_cell = num_cells-1;    
    let mut goal_reached = true;
    
    //The directions we can move
    let directions: &[(i32,i32)] = &[
        (0, -1),    //North
        (1, 0),     //East
        (0, 1),     //South
        (-1, 0),    //West
    ];

    let mut last_byte_dropped: Option<usize> = None;
    while goal_reached {
        goal_reached = false;
        last_byte_dropped = bytes_to_drop.pop_front();

        if last_byte_dropped.is_none() { break }

        corrupted_memory_space[last_byte_dropped.unwrap()] = true;

        let mut visited: Vec<bool> = vec![false; num_cells];
        let mut route: Vec<i32> = vec![-1; num_cells];
        let mut queue: VecDeque<Edge> = VecDeque::new();
        //Add the start cell
        queue.push_back(Edge::new(start_cell, start_cell));

        while let Some(next) = queue.pop_front() {
            //Mark the parent of the cell
            route[next.to] = next.from as i32;

            //If the goal is reached, stop
            if next.to == goal_cell {
                goal_reached = true;
                break;
            }

            //For each potential direction, add an edge to the queue
            //as long as we haven't been there before and its within
            //bounds
            for dir in directions {
                //Edges from next.to to visit
                let x = next.to % mem_width;
                let y = (next.to - x) / mem_height;
                let (to_x, to_y) = (x as i32 + dir.0, y as i32 + dir.1);
                //Bounds check
                if to_x >= 0 && to_x < mem_width as i32
                && to_y >= 0 && to_y < mem_height as i32 {
                    let to_cell = (to_y as usize * mem_height) + to_x as usize;
                    //Check whether we've visited the cell before or if it is corrupted
                    if !visited[to_cell] && !corrupted_memory_space[to_cell] {
                        queue.push_back(Edge::new(next.to, to_cell));
                        //Important: cell is marked as visited before its processed 
                        //to reduce number of edges being processed
                        visited[to_cell] = true;
                    }
                }
            }
        }
    }

    if let Some(last_byte) = last_byte_dropped {
        let x = last_byte % mem_width;
        let y = (last_byte - x) / mem_height;
        Ok(format!("{},{}", x, y))
    } else {
        anyhow::bail!("Failed to get blocked from goal!")
    }
}