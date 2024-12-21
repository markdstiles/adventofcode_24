//https://adventofcode.com/2024/day/3

use std::{fs::File, io::{BufRead, BufReader}};

fn extract_digits(text: &str) -> Option<(i32, i32)> {
    //Look for digit from start of slice to next ','
    if let Some(comma_index) = text.find(',') {
        //Extract the first digit to the ,
        if let Ok(first_num) = text[..comma_index].parse::<i32>() {
            //Find next ')' if there is one
            if let Some(bracket_index) = text.find(')') {
                if let Ok(second_num) = text[comma_index + 1..bracket_index].parse::<i32>() {
                    //If we did find ')' extract digit from , .. )
                    return Some((first_num, second_num))
                }
            }
        }
    }

    //Otherwise return nothing
    None
}

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 3 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day3.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut running_total = 0_i64;

    for line in reader.lines() {
        let line = line?;
        let mut searching = true;
        let mut remaining = &line[..];

        while searching {
            if let Some(mul_index) = remaining.find("mul(") {
                remaining = &remaining[mul_index + 4..];

                //Find mul(
                if let Some(next_mul_index) = remaining.find("mul(") {    
                    //Values will be between this mul( and the next mul (
                    if let Some((x, y)) = extract_digits(&remaining[..next_mul_index]) {
                        running_total += (x * y) as i64;
                    }
                } else {
                    //No more mul( in the line, get last digits & stop searching
                    if let Some((x, y)) = extract_digits(remaining) {
                        running_total += (x * y) as i64;
                    }

                    searching = false;
                }
            } else {
                //No mul( found
                searching = false;
            }
        }
    }

    Ok(running_total)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 3 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day3.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut running_total = 0_i64;
    let mut is_doing = true;

    for line in reader.lines() {
        let line = line?;
        let mut searching_for_blocks = true;
        let mut remaining = &line[..];

        while searching_for_blocks {

            //Check if we are in a doing block, either at the start or from previous line
            if is_doing {
                let mut block: &str = remaining;
                //Look for when the next don't() is, this defines end of block
                if let Some(dont_index) = remaining.find("don't()") {
                    block = &remaining[..dont_index];
                    remaining = &remaining[dont_index + 7..];
                    
                    //We found a block, look for next block next time
                    is_doing = false;
                } else {
                    //No further don't() on this line so we can process the remainder of the line, but still in a do() block for the next line
                    searching_for_blocks = false;
                }

                let mut searching_for_mul = true;

                while searching_for_mul {
                    //Search for mul( within this block
                    if let Some(mul_index) = block.find("mul(") {
                        block = &block[mul_index + 4..];
        
                        //Find mul(
                        if let Some(next_mul_index) = block.find("mul(") {    
                            //Values will be between this mul( and the next mul (
                            if let Some((x, y)) = extract_digits(&block[..next_mul_index]) {
                                running_total += (x * y) as i64;
                            }
                        } else {
                            //No more mul( in the line, get last digits & stop searching
                            if let Some((x, y)) = extract_digits(block) {
                                running_total += (x * y) as i64;
                            }
        
                            searching_for_mul = false;
                        }
                    } else {
                        //No mul( found
                        searching_for_mul = false;
                    }
                }
            } else {
                //Otherwise look for next do()
                if let Some(do_index) = remaining.find("do()") {
                    remaining = &remaining[do_index + 4..];
                    is_doing = true;
                } else {
                    //Nothing left to 'do' on this line
                    searching_for_blocks = false;
                }
            }
        }
    }

    Ok(running_total)
}