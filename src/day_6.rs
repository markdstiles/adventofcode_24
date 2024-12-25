//https://adventofcode.com/2024/day/6

use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 6 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day6.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let directions:[(i32, i32); 4] = [ 
        (0, -1), //UP
        (1, 0),  //RIGHT
        (0, 1),  //DOWN
        (-1, 0)  //LEFT
    ];
    let mut curr_direction: usize = 0;
    let mut curr_position: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<char>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        map.push(line.as_bytes().iter().map(|b| *b as char).collect());
        if let Some(index) = line.find('^') {
            curr_position = (index, i);
        }
    }

    let width = map[0].len();
    let height = map.len();

    //Include starting position
    let mut steps = 1;
    map[curr_position.1][curr_position.0] = 'v';

    while let Some((x, y)) = add(curr_position, directions[curr_direction]) {
        //Check we are still in bounds
        if x < width && y < height {
            //If we hit something then change direction
            if map[y][x] == '#' {
                curr_direction += 1;
                if curr_direction > 3 {
                    curr_direction = 0;
                }
            } else {
                //Move
                if map[y][x] != 'v' {
                    //If we haven't visited this before then count the step
                    steps += 1;
                }
                map[y][x] = 'v';
                curr_position = (x, y);
            }
        } else {
            //We're out of bounds
            break;
        }
    }

    if false /* display map */ {
        for row in map {
            for c in row {
                print!("{c}");
            }
            println!();
        }
    }

    Ok(steps as i64)
}

fn add(pos: (usize, usize), offset: (i32, i32)) -> Option<(usize, usize)> {
    let new_pos = (pos.0 as i32 + offset.0, pos.1 as i32 + offset.1);

    if new_pos.0 >= 0 && new_pos.1 >= 0 {
        return Some((new_pos.0 as usize, new_pos.1 as usize))
    }

    None
}

fn display_direction(direction: usize) -> char {
    match direction {
        0 => '^',
        1 => '>',
        2 => 'v',
        3 => '<',
        _ => { panic!("Whoops! {direction}")},
    }
}

fn display_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

fn add_obstacles(map: &mut Vec<Vec<char>>, obstacle_positions: &HashSet<(usize, usize)>) {
    for (x, y) in obstacle_positions {
        map[*y][*x] = 'O';
    }
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 6 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day6.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let directions:[(i32, i32); 4] = [ 
        (0, -1), //UP
        (1, 0),  //RIGHT
        (0, 1),  //DOWN
        (-1, 0)  //LEFT
    ];
    let mut curr_direction: usize = 0;
    let mut curr_position: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<char>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        map.push(line.as_bytes().iter().map(|b| *b as char).collect());
        if let Some(index) = line.find('^') {
            curr_position = (index, i);
        }
    }

    let width = map[0].len();
    let height = map.len();

    //Include starting position
    map[curr_position.1][curr_position.0] = '^';
    let mut changed_direction = false;
    let mut obstacle_positions: HashSet<(usize, usize)> = HashSet::new();

    while let Some((x, y)) = add(curr_position, directions[curr_direction]) {
        //Check we are still in bounds
        if x < width && y < height {
            //If we hit something then change direction
            if map[y][x] == '#' {
                curr_direction += 1;
                if curr_direction > 3 {
                    curr_direction = 0;
                }
                changed_direction = true;
            } else {
                //Try and trap the guard by adding an obstacle in the position we're about to move to
                //Store hashset of the current position relative to the wall they hit, 
                //then to detect a loop if we arrive at the same position again then we have looped
                map[y][x] = '#';

                let obstacle_position = (x, y);
                let mut target_position = curr_position;
                let mut target_direction = curr_direction;
                let mut wall_collisions: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
                while let Some((x, y)) = add(target_position, directions[target_direction]) {
                    //Check we are still in bounds
                    if x < width && y < height {
                        //If we hit something then change direction
                        if map[y][x] == '#' {
                            target_direction += 1;
                            if target_direction > 3 {
                                target_direction = 0;
                            }
                            //Record current position and wall position
                            let wall_collision = (target_position, (x, y));
                            if wall_collisions.contains(&wall_collision) {
                                //We have arrived back at a position we've been in before - we have looped
                                obstacle_positions.insert(obstacle_position);
                                break;
                            }
                            wall_collisions.insert(wall_collision);
                        } else {
                            //Move
                            target_position = (x, y);
                        }
                    } else {
                        //We're out of bounds
                        break;
                    }
                }

                //Move
                if changed_direction {
                    map[curr_position.1][curr_position.0] = display_direction(curr_direction);
                    changed_direction = false;
                }
                map[y][x] = display_direction(curr_direction);
                curr_position = (x, y);
            }
        } else {
            //We're out of bounds
            break;
        }
    }

    add_obstacles(&mut map, &obstacle_positions);
    display_map(&map);

    Ok(obstacle_positions.len() as i64)
}