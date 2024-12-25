//https://adventofcode.com/2024/day/7

use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 7 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day7.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut total_calibration_result = 0;
    for line in reader.lines() {
        let line = line?;

        if let Some((test_value, number_str)) = line.split_once(": ") {
            let test_value: i64 = test_value.parse()?;
            let numbers: Vec<i64> = number_str.split(" ").map(|s| s.parse().unwrap()).collect();

            let mut pass = false;
            let iterations = 2_i32.pow(numbers.len() as u32 - 1);

            for i in 0..iterations {

                let mut result = numbers[0];
                let mut equation = numbers[0].to_string();

                for (num_index, number) in numbers.iter().enumerate().skip(1) {
                    let do_addition = 1 << (num_index-1) & i != 0;

                    result = if do_addition { 
                        equation = format!("{} + {}", equation, *number);
                        result + *number
                    } else { 
                        equation = format!("{} * {}", equation, *number);
                        if *number == 1 {
                            result
                        } else {
                            result * *number
                        }
                    };
                }
                pass = result == test_value;

                if pass {
                     println!("Equation passes: {test_value}={equation}");
                     break;
                }
            }

            if pass {
                total_calibration_result += test_value;
            }
        }
    }

    Ok(total_calibration_result)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 7 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day7.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    Ok(0)
}