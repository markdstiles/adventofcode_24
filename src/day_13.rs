//https://adventofcode.com/2024/day/13

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 13 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day13.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let mut reader = BufReader::new(file);

    let mut total_cost: i32 = 0;

    loop {
        let mut button_a_line = String::new();

        if reader.read_line(&mut button_a_line).is_err() {
            //Probably end of file
            break;
        }
        if button_a_line.is_empty() {
            break;
        }

        let mut button_b_line = String::new();
        reader.read_line(&mut button_b_line)?;
        let mut target_line = String::new();
        reader.read_line(&mut target_line)?;
        //Read blank line
        reader.read_line(&mut target_line)?;

        
        let button_a = MachinePos::from(button_a_line.trim().split('+').skip(1).map(|s| { 
            if let Some(idx) = s.find(',') {
                s[..idx].parse::<i32>().unwrap()
            } else {
                s.parse::<i32>().unwrap()
            }
        }).collect::<Vec<i32>>());

        let button_b = MachinePos::from(button_b_line.trim().split('+').skip(1).map(|s| { 
            if let Some(idx) = s.find(',') {
                s[..idx].parse::<i32>().unwrap()
            } else {
                s.parse::<i32>().unwrap()
            }
        }).collect::<Vec<i32>>());

        let target = MachinePos::from(target_line.trim().split('=').skip(1).map(|s| { 
            if let Some(idx) = s.find(',') {
                s[..idx].parse::<i32>().unwrap()
            } else {
                s.parse::<i32>().unwrap()
            }
        }).collect::<Vec<i32>>());

        let mut result = test_machine(button_a, button_b, target);

        if !result.is_empty() {
            result.sort_by_key(|o| o.cost);
            total_cost += result[0].cost;
        }
    }
    
    Ok(total_cost as i64)
}

#[derive(Debug)]
struct MachinePos {
    x: i32,
    y: i32,
}

impl MachinePos {
    fn new(x: i32, y: i32) -> MachinePos {
        MachinePos {
            x, y,
        }
    }

    fn from(vals: Vec<i32>) -> MachinePos {
        if vals.len() != 2 {
            panic!()
        }

        MachinePos {
            x: vals[0],
            y: vals[1],
        }
    }
}

#[derive(Debug)]
struct MachineResult {
    button_a_count: i32,
    button_b_count: i32,
    cost: i32,
}

fn test_machine(button_a: MachinePos, button_b: MachinePos, target: MachinePos) -> Vec<MachineResult> {
    let button_a_cost = 3;
    let button_b_cost = 1;

    let primary_button;
    let secondary_button;
    let primary_is_button_a;
    
    //Determine which of the buttons is most efficient
    if button_b.x > button_a.x {
        primary_is_button_a = false;
        primary_button = &button_b;
        secondary_button = &button_a;
    } else {
        primary_is_button_a = true;
        primary_button = &button_a;
        secondary_button = &button_b;
    }

    let mut results: Vec<MachineResult> = Vec::new();
    
    let mut iteration = target.x / primary_button.x;
    while iteration >= 0 {
        //Does button A & Button B combination allow us to reach the target?
        let primary_result = primary_button.x * iteration;
        let shortfall = target.x - primary_result;
        //if there is no remainder then we have a potential combination of button a & b presses
        if shortfall % secondary_button.x == 0 {
            //Now check if y axis also aligns
            let primary_button_presses = iteration;
            let secondary_button_presses = shortfall / secondary_button.x;

            if (primary_button_presses * primary_button.y) 
            + (secondary_button_presses * secondary_button.y) == target.y {
                //We have a combination that will get us to the target on both axis
                if primary_is_button_a {
                    let button_a_count = primary_button_presses;
                    let button_b_count = secondary_button_presses;
                    let cost = (button_a_cost * button_a_count) + (button_b_cost * button_b_count);
                    results.push(MachineResult { button_a_count, button_b_count, cost });
                } else {
                    let button_b_count = primary_button_presses;
                    let button_a_count = secondary_button_presses;
                    let cost = (button_a_cost * button_a_count) + (button_b_cost * button_b_count);
                    results.push(MachineResult { button_a_count, button_b_count, cost });
                }
            }
        }

        iteration -= 1;
    }
    
    results
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 13 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day13.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
   
    //To do look at Cramers Rule

    Ok(0)
}