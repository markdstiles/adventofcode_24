//https://adventofcode.com/2024/day/7

use std::{fs::File, io::{BufRead, BufReader}};

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

                     //Short circuit
                     if result > test_value {
                        break;
                    }
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

    let mut part1_total = 0;
    let mut part2_input: Vec<(i64, Vec<i64>)> = Vec::new();

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

                     //Short circuit
                     if result > test_value {
                        break;
                    }
                }
                pass = result == test_value;

                if pass {
                    part1_total += test_value;
                     break;
                }
            }

            if !pass {
                part2_input.push((test_value, numbers.clone()));
            }
        }
    }

    let mut total_calibration_result = part1_total;
    
    for (test_value, numbers) in part2_input {

        match apply_expression(test_value, 0, Op::Concat, &numbers[..]) {
            Outcome::Pass(expression) => {
                println!("Expression passed: 0{expression} = {test_value}");
                total_calibration_result += test_value;
                continue;
            },
            Outcome::Fail(expression) => println!("Expression failed: 0{expression} = {test_value}"),
        }
        match apply_expression(test_value, 0, Op::Add, &numbers[..]) {
            Outcome::Pass(expression) => {
                println!("Expression passed: 0{expression} = {test_value}");
                total_calibration_result += test_value;
                continue;
            },
            Outcome::Fail(expression) => println!("Expression failed: 0{expression} = {test_value}"),
        }
        match apply_expression(test_value, 1, Op::Multiply, &numbers[..]) {
            Outcome::Pass(expression) => {
                println!("Expression passed: 1{expression} = {test_value}");
                total_calibration_result += test_value;
                continue;
            },
            Outcome::Fail(expression) => println!("Expression failed: 1{expression} = {test_value}"),
        }
    }

    Ok(total_calibration_result)
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Multiply,
    Concat
}

#[derive(PartialEq)]
enum Outcome {
    Pass(String),    
    Fail(String),
}

fn apply_expression(test_value: i64, input: i64, operator: Op, numbers: &[i64]) -> Outcome {
    let result = match operator {
        Op::Add => input + numbers[0],
        Op::Multiply => input * numbers[0],
        Op::Concat => format!("{}{}", input, numbers[0]).parse::<i64>().unwrap(),
    };

    let expression = match operator {
        Op::Add => format!(" + {}", numbers[0]),
        Op::Multiply => format!(" * {}", numbers[0]),
        Op::Concat => format!(" || {}", numbers[0]),
    };

    //If result is too big...
    if result > test_value {
        return Outcome::Fail(expression);
    }

    //If it's the last number in the list...
    if numbers.len() == 1 {
        if result == test_value {
            return Outcome::Pass(expression);
        } else {
            return Outcome::Fail(expression);
        }
    }

    if let Outcome::Pass(pass_expression) = apply_expression(test_value, result, Op::Concat, &numbers[1..]) {
        return Outcome::Pass(format!("{}{}", expression, pass_expression));
    }

    if let Outcome::Pass(pass_expression) = apply_expression(test_value, result, Op::Add, &numbers[1..]) {
        return Outcome::Pass(format!("{}{}", expression, pass_expression));
    }

    match apply_expression(test_value, result, Op::Multiply, &numbers[1..]) {
        Outcome::Pass(pass_expression) => Outcome::Pass(format!("{}{}", expression, pass_expression)),
        Outcome::Fail(fail_expression) => Outcome::Fail(format!("{}{}", expression, fail_expression)),
    }
}