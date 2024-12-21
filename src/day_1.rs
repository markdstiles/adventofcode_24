//https://adventofcode.com/2024/day/1

use std::{fs::File, io::{BufRead, BufReader}};

pub fn run() -> anyhow::Result<()> {
    println!("Day 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day1.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    
    let mut left = vec![0_i32; 1000];
    let mut right = vec![0_i32; 1000];

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        
        //Split line into two and push into lists
        let parts: Vec<&str> = line.split("   ").collect();

        left[index] = parts[0].parse::<i32>()?;
        right[index] = parts[1].parse::<i32>()?;
    }
    
    //Sort lists
    println!("Sorting lists...");
    left.sort();
    right.sort();

    //Compare each item in the list to the same index in the other and subtract one from the other
    let mut running_total = 0;
    for i in 0..1000 {
        let l = left[i];
        let r = right[i];

        //Keep a running total of the sum of the absolute distance values
        running_total += i32::abs(l - r);

    }
    
    println!("Part 1 - Total: {running_total}");

    //Part 2...

    //Both lists are already sorted from Part 1...
    //Starting with the first number in the left list
    let mut left_index = 0;
    //and the first number in the right list
    let mut right_index = 0;

    let mut left_histo = vec![0_i32; 1000];

    while left_index < 1000 && right_index < 1000 {
        
        match right[right_index].cmp(&left[left_index]) {
            std::cmp::Ordering::Equal => {
                println!("MATCH! [{left_index}]{} = [{right_index}]{}", left[left_index], right[right_index]);
                //The left and right numbers match
                //Increment our histogram count
                left_histo[left_index] += 1;

                //Move the next item in the right list
                right_index += 1;
            },
            std::cmp::Ordering::Greater => {
                println!("Greater-than [{left_index}]{} < [{right_index}]{}", left[left_index], right[right_index]);
                //We've moved to a number in the right list that is larger than the current item in the left list
                //we need to look for a number in the left list that matches or is greater than the number in the right list
                //Move to the next number in the left list
                left_index += 1;
            },
            std::cmp::Ordering::Less => {
                println!("Less-than [{left_index}]{} > [{right_index}]{}", left[left_index], right[right_index]);
                //If the number in the right list is now smaller than the number in the left list
                //we didn't find it in the left list so move to the next number in the right list
                right_index += 1;
            }
        }
    }

    let mut running_total: i64 = 0;

    for (index, val) in left_histo.into_iter().enumerate() {
        running_total += (left[index] * val) as i64;
    }

    println!("Part 2 - Total: {running_total}");

    Ok(())
}