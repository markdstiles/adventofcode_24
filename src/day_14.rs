//https://adventofcode.com/2024/day/14

use crate::input_utils::parse_formatted;
use std::{fmt::Display, fs::File, io::{BufRead, BufReader}, ops::AddAssign};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vector2D {
    x: i32,
    y: i32,
}

impl Vector2D {
    fn new(x: i32, y: i32) -> Vector2D {
        Vector2D { x, y }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Rect {
    fn new(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
        Rect { left, top, right, bottom }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect(L:{},T:{},R:{},B:{})", self.left, self.top, self.right, self.bottom)
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

    Ok(0)
}