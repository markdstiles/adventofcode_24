//https://adventofcode.com/2024/day/10

use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 10 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day10.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
    
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    let map: Vec<Vec<i8>> = reader.lines()
        .enumerate()
        .map(|(row, line)| 
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(col, c)| {
                    let height = c.to_digit(10).unwrap() as i8;
                    if height == 0 {
                        trailheads.push((col, row));
                    }
                    height
                }).collect::<Vec<i8>>()
            )
        .collect();

    let map_width = map[0].len() as i32;
    let map_height = map.len() as i32;
    let bounds = (map_width, map_height);

    let mut summits_reached: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    trailheads.iter().for_each(|pos| {
        walk_trail_path(&map, bounds, *pos, *pos, (0, 1), &mut summits_reached);
        walk_trail_path(&map, bounds, *pos, *pos, (1, 0), &mut summits_reached);
        walk_trail_path(&map, bounds, *pos, *pos, (0, -1), &mut summits_reached);
        walk_trail_path(&map, bounds, *pos, *pos, (-1, 0), &mut summits_reached);
    });

    summits_reached.iter().for_each(|(origin, summit_reached)| println!("{:?}->{:?}", *origin, *summit_reached));

    Ok(summits_reached.len() as i64)
}

fn walk_trail_path(map: &Vec<Vec<i8>>, bounds: (i32, i32), pos: (usize, usize), origin: (usize, usize), direction: (i32, i32), summits_reached: &mut HashSet<((usize, usize), (usize, usize))>) {
    //Recursively walk a trail until we can't climb any further or reach the summit (height 9)
    let current_height = map[pos.1][pos.0];
    let new_pos = (pos.0 as i32 + direction.0, pos.1 as i32 + direction.1);

    //Bounds check
    if new_pos.0 >= 0
    && new_pos.0 < bounds.0
    && new_pos.1 >= 0
    && new_pos.1 < bounds.1 {
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        let origin_direction = (-direction.0, -direction.1);

        let new_height = map[new_pos.1][new_pos.0];

        //Can we climb to the new position?
        if new_height - current_height == 1 {
            if new_height == 9 {
                summits_reached.insert((origin, new_pos));
            } else {
                //Try and move in other 3 directions except the one we just came from
                if origin_direction != (0, 1) {
                    walk_trail_path(map, bounds, new_pos, origin, (0, 1), summits_reached);
                }
                if origin_direction != (1, 0) {
                    walk_trail_path(map, bounds, new_pos, origin,  (1, 0), summits_reached);
                }
                if origin_direction != (0, -1) {
                    walk_trail_path(map, bounds, new_pos, origin, (0, -1), summits_reached);
                }
                if origin_direction != (-1, 0) {
                    walk_trail_path(map, bounds, new_pos, origin, (-1, 0), summits_reached);
                }
            }
        }
    }
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 10 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day10.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
   
   Ok(0)
}