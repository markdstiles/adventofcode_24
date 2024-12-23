//https://adventofcode.com/2024/day/6

use std::{fs::File, io::{BufRead, BufReader}, ops::Index};

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

    for row in map {
        for c in row {
            print!("{c}");
        }
        println!();
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

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 6 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day6.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    
    Ok(0)
}