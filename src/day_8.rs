//https://adventofcode.com/2024/day/8

use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 8 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day8.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut antenna_locations: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let map_width = lines[0].len() as i32;
    let map_height = lines.len() as i32;

    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c != '.' {
                antenna_locations.entry(c)
                    .and_modify(|v| v.push((col as i32, row as i32)))
                    .or_insert(Vec::from([(col as i32, row as i32)]));
            }
        })
    });

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    //Calculate position of anti-nodes
    for (_, locations) in antenna_locations.iter() {
         for i in 0..locations.len()-1 {
            let first = locations[i];
            for second in locations[i+1..].iter() {
                //We don't need to calculate the distance, instead just extract the vector representation between the two points
                //Then translate the point by the vector in either direction
                let vector = (first.0 - second.0, first.1 - second.1);
                let first_antinode = (first.0 + vector.0, first.1 + vector.1);
                let second_antinode = (second.0 - vector.0, second.1 - vector.1);

                //Record the antinode if its in the bounds of the map
                if first_antinode.0 >= 0 
                && first_antinode.0 < map_width
                && first_antinode.1 >= 0
                && first_antinode.1 < map_height {
                    antinodes.insert(first_antinode);
                }

                if second_antinode.0 >= 0 
                && second_antinode.0 < map_width
                && second_antinode.1 >= 0
                && second_antinode.1 < map_height {
                    antinodes.insert(second_antinode);
                }
            }
         }
    }

    Ok(antinodes.len() as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 8 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day8.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    Ok(0)
}