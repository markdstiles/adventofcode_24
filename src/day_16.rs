//https://adventofcode.com/2024/day/16

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 16 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day16.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    Ok(0)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 16 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day16.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    Ok(0)
}