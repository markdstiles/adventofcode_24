//https://adventofcode.com/2024/day/2

use std::{fs::File, io::{BufRead, BufReader}};

fn is_safe(readings: &[i32]) -> bool {
    let mut previous_reading = readings[0];
    let is_asc = readings[1] > previous_reading;
    let mut is_safe = true;

    for curr_reading in readings.iter().skip(1) {
        is_safe = (is_asc && previous_reading < *curr_reading && *curr_reading - previous_reading >= 1 && *curr_reading - previous_reading <= 3) 
            || 
            (!is_asc && previous_reading > *curr_reading && previous_reading - *curr_reading >= 1 && previous_reading - *curr_reading <= 3);

        //If still not safe...
        if !is_safe {
            break;
        }

        previous_reading = *curr_reading;
    }

    is_safe
}

pub fn run() -> anyhow::Result<()> {
    println!("Day 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day2.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut num_safe_readings = 0;
    
    for line in reader.lines() {
        let report = line?;
        
        //With each report split the line into readings
        let readings: Vec<i32> = report
            .split(" ")
            .map(|r| r.parse::<i32>().unwrap())
            .collect();

        //Determine whether the reading is safe based on the following rules:
        // - All numbers are either ascending or descending
        // - Distances between numbers are between 1..3 but no more or less

        if is_safe(&readings) {
            num_safe_readings += 1;
        }
    }

    println!("Part 1 - {num_safe_readings} readings are safe!");
    
    //Part 2 - the problem dampener....
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let do_debug = false;

    for line in reader.lines() {
        let report = line?;
        
        //With each report split the line into readings
        let readings: Vec<i32> = report
            .split(" ")
            .map(|r| r.parse::<i32>().unwrap())
            .collect();

        //Determine whether the reading is safe based on the following rules:
        // - All numbers are either ascending or descending
        // - Distances between numbers are between 1..3 but no more or less
        //Problem dampener allows for ONE violation of the above rules

        if is_safe(&readings) {
            //num_safe_readings += 1;
            if do_debug {
                print!("Was safe first time. ");
            }
        }
        else {
            //Readings weren't safe; Apply the problem dampener...
            //Try removing an item, if the remaining readings are safe then stop
            if do_debug {
                print!("Was not safe first time...");
            }
            for i in 0..readings.len() {
                let mut new_readings = readings.clone();
                new_readings.remove(i);

                if is_safe(&new_readings) {
                    if do_debug {
                        print!("removing reading {} ({}) made it safe. ", i+1, readings[i]);
                    }
                    num_safe_readings += 1;
                    break;
                }
            }
        }
        if do_debug { 
            println!("Readings: {:?}", readings);
        }
    }

    println!("Part 2 - {num_safe_readings} readings are now also safe!");

    Ok(())
}