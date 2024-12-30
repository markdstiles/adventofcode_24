//https://adventofcode.com/2024/day/11

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 11 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day11.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut stones: Vec<u64> = reader.lines()
        .next()
        .map(|line| 
            line.unwrap()
            .split(' ')
            .map(|s| 
                s.parse::<u64>().unwrap()
            ).collect::<Vec<u64>>()
        ).unwrap();
    
    let stop_at_blinks = 25;
    let mut blink_count = 0;

    loop {
        if blink_count == stop_at_blinks {
            break;
        }

        let stone_count = stones.len();
        let mut inserted_count = 0;
        for i in 0..stone_count {
            let stone = &mut stones[i + inserted_count];

            if *stone == 0 {
                //Replace with 1
                *stone = 1;
            } else {
                let stone_str = stone.to_string();

                if stone_str.len() % 2 == 0 {
                    //Even number of digits...split in two
                    let (left, right) = stone_str.split_at(stone_str.len() / 2);
                    let (left, right) = (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap());
                    *stone = left;
                    inserted_count += 1;
                    stones.insert(i + inserted_count, right);
                } else {
                    //Replace stone value with its value multiplied by 2024
                    *stone *= 2024;
                }
            }
        }

        blink_count += 1;
    }
    
    Ok(stones.len() as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 11 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day11.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
   
   Ok(0)
}