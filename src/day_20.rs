//https://adventofcode.com/2024/day/20

use std::{fs::File, io::{BufRead, BufReader}, thread::current};
use crate::misc_types::{Point, Rect};

pub fn do_part1() -> anyhow::Result<usize> {
    println!("Day 20 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day20.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    //Cell types
    const EMPTY_SPACE: i32 = -1;
    const WALL: i32 = -2;
    const START: i32 = -3;
    const END: i32 = -4;

    //Load the map
    let mut start_pos: Point<usize> = Point::new(0, 0);
    let mut end_pos: Point<usize> = Point::new(0,0);
    let mut map: Vec<Vec<i32>> = reader.lines().enumerate().map(|(y, line)| 
        line.unwrap().chars().enumerate().map(|(x, c)| 
            match c {
                '#' => WALL,
                'S' => {
                    start_pos = Point::new(x, y);
                    START
                },
                'E' => {
                    end_pos = Point::new(x, y);
                    END
                },
                _ => EMPTY_SPACE,
            }
        ).collect()
    ).collect();

    let bounds: Rect = Rect::new(
        0, 
        0, 
        map[0].len() as i32, 
        map.len() as i32
    );

    let directions = [
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];
    
    //From the start position, walk the racetrack marking the distance of each step from the start
    //Note any locations where the wall is one cell thick and there's empty space on the otherside
    let mut current_direction: Point<i32> = directions[0];
    let mut current_pos: Point<usize> = start_pos;
    let mut distance = 0;
    let mut cheat_points: Vec<(Point<usize>, Point<usize>)> = vec![];

    while current_pos != end_pos {
        if current_pos != start_pos {
            map[current_pos.y][current_pos.x] = distance;
        }

        let mut next_direction = current_direction;
        let prev_cell_dir: Point<i32> = Point::new(-current_direction.x, -current_direction.y);
        directions.iter()
            .filter(|&&d| d != prev_cell_dir)
            .for_each(|&d| {
                let probe: Point<i32> = d + current_pos.into();

                if bounds.is_inside(probe) {
                    let u_probe: Point<usize> = probe.into();
                    match map[u_probe.y][u_probe.x] {
                        EMPTY_SPACE => next_direction = d,
                        WALL => {
                            let next_cell = probe + d;
                            //If cell beyond wall is in bounds
                            if next_cell.inside(bounds) {
                                let u_next_cell: Point<usize> = next_cell.into();
                                //If cell beyond wall is empty space
                                if map[u_next_cell.y][u_next_cell.x] == EMPTY_SPACE {
                                    //Potential cheat point
                                    cheat_points.push((current_pos, u_next_cell));
                                }
                            }
                        },
                        _ => {},
                    }
                }
            });

        let mut next_pos: Point<i32> = current_pos.into();
        next_pos += next_direction;

        if next_pos.inside(bounds) {
            current_pos = next_pos.into();
            current_direction = next_direction;
            distance += 1;
        } else {
            println!("Dist:{distance}, pos:{}, next_pos:{} dir:{} next_dir:{}", current_pos, next_pos, current_direction, next_direction);
            panic!("Left the racetrack and crashed!");
        }
    }

    //Calculate the distances & rank
    let num_cheats = cheat_points.iter()
        .map(|&(from, to)| map[to.y][to.x] - map[from.y][from.x])
        .filter(|&distance| distance > 100)
        .count();

    Ok(num_cheats)
}

fn print_map(map: &[Vec<i32>]) {
    map.iter().for_each(|row| {
        row.iter().for_each(|&column| 
            {
                let c_str = if column >= 0 {
                    column.to_string() 
                } else {
                    String::from(".")
                };

                print!("{}",
                    match column {
                        -2 => "#",
                        -3 => "S",
                        -4 => "E",
                        _ => c_str.as_str(),
                    }
                );
            }
        );
        println!();
    });
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 20 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day20.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    Ok(0)
}