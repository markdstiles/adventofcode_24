//https://adventofcode.com/2024/day/14

use crate::input_utils::parse_formatted;
use crate::misc_types::*;
use std::{fs::File, io::{BufRead, BufReader, Write}};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Robot {
    position: Vector2D,
    velocity: Vector2D,
}

impl Robot {
    fn new(pos_x: i32, pos_y: i32, velocity_x: i32, velocity_y: i32) -> Robot {
        Robot { 
            position: Vector2D::new(pos_x, pos_y),
            velocity: Vector2D::new(velocity_x, velocity_y),
         }
    }

    fn ensure_position_wrapped(&mut self, map_width: i32, map_height: i32) {
        if self.position.x < 0 {
            self.position.x += map_width;
        }
        if self.position. y < 0 {
            self.position.y += map_height;
        }
        if self.position.x >= map_width {
            self.position.x -= map_width;
        }
        if self.position.y >= map_height {
            self.position.y -= map_height;
        }
    }
}

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 14 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day14.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
    let mut robots: Vec<Robot> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let robot_params: Vec<i32> = parse_formatted(line, "p={},{} v={},{}".into())
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        robots.push(Robot::new(
            robot_params[0],
            robot_params[1],
            robot_params[2],
            robot_params[3],
        ));
    }

    let width: i32 = 101;
    let height: i32 = 103;

    for _ in 0..100 {
        for robot in &mut robots {
            robot.position += robot.velocity;
            robot.ensure_position_wrapped(width, height);
        }
    }

    let quadrants: [Rect; 4] = [
        Rect::new(0,0, 50, 51),
        Rect::new(51,0, 101, 51),
        Rect::new(0,52, 50, 103),
        Rect::new(51,52, 101, 103),
    ];

    let mut safety_factor = 1;
    
    for quadrant in quadrants {
        safety_factor *= robots.iter()
            .filter(|&r| r.position.x >= quadrant.left 
                && r.position.x < quadrant.right 
                && r.position.y >= quadrant.top 
                && r.position.y < quadrant.bottom 
            ).count();
    }

    Ok(safety_factor as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 14 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day14.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut robots: Vec<Robot> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let robot_params: Vec<i32> = parse_formatted(line, "p={},{} v={},{}".into())
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        robots.push(Robot::new(
            robot_params[0],
            robot_params[1],
            robot_params[2],
            robot_params[3],
        ));
    }

    let width: i32 = 101;
    let height: i32 = 103;
    let mut map: Vec<Vec<i32>> = vec![vec![0; 101]; 103];

    let mut output_file = std::env::current_dir()?;
    output_file.push("output\\day14_map.txt");

    println!("Writing output to {}", output_file.display());

    let mut file = File::create(output_file.clone())?;

    //Every 103 seconds from 57 secs onwards seems to be where a pattern is forming
    //Every 101 seconds from 98 secs onwards seems to be where the pattern is forming

    //Mark-I eyeball evaluation of output!
    for iteration in 0..10000 {
        for robot in &mut robots {
            if map[robot.position.y as usize][robot.position.x as usize] != 0 {
                map[robot.position.y as usize][robot.position.x as usize] -= 1;
            }

            robot.position += robot.velocity;
            robot.ensure_position_wrapped(width, height);

            map[robot.position.y as usize][robot.position.x as usize] += 1;
        }

        if (iteration - 98) % 101 == 0 {
            write_map_to_file(&mut file, &map, iteration)?;
        }
    }

    Ok(7371)
}

fn write_map_to_file(file: &mut File, map: &Vec<Vec<i32>>, iteration: i32) -> anyhow::Result<()> {
    writeln!(file, "Iteration: {iteration}")?;

    for row in map {
        for col in row {
            write!(file, "{}", if *col == 0 { " " } else { "*" })?;
        }
        writeln!(file)?;
    }

    Ok(())
}