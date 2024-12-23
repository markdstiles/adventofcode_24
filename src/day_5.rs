//https://adventofcode.com/2024/day/5

use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 5 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day5.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut page_number_rules: HashSet<String> = HashSet::new();
    let mut running_total = 0;

    for line in reader.lines() {
        let line = line?;

        if line.contains("|") {
            //It's a page ordering rule
            let relationship = line.replace("|", ",");

            //Store relationship
            page_number_rules.insert(relationship);
        } else if line.contains(",") {
            //Check each pair of numbers in the page sequence follows the ordering rules
            let page_numbers: Vec<&str> = line.split(",").collect();

            let mut not_valid = false;
            for i in 0..page_numbers.len()-1 {
                //If the pair of numbers doesn't exist in the page number rules then the sequence is invalid
                let pair = format!("{},{}", page_numbers[i], page_numbers[i+1]);
                if !page_number_rules.contains(&pair) {
                    not_valid = true;
                    break;
                }
            }

            //If the sequence is valid, find the middle number and add it to the running total
            if !not_valid {
                running_total += page_numbers[page_numbers.len() / 2].parse::<i32>()? as i64;
            }
        }
    }

    Ok(running_total)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 5 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day5.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut page_number_rules: HashSet<String> = HashSet::new();
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut running_total = 0;

    for line in reader.lines() {
        let line = line?;

        if line.contains("|") {
            //It's a page ordering rule
            let relationship = line.replace("|", ",");

            //Store relationship
            page_number_rules.insert(relationship);

            //Add to graph
            if let Some((lower, higher)) = line.split_once("|")
                .map(|t| (t.0.parse::<i32>().unwrap(), t.1.parse::<i32>().unwrap())) {
                    graph
                        .entry(lower)
                        .and_modify(|hset| {
                            hset.insert(higher);
                        })
                        .or_insert(HashSet::from([higher]));
            }
        } else if line.contains(",") {
            //Check each pair of numbers in the page sequence follows the ordering rules
            let page_numbers: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

            let mut not_valid = false;
            for i in 0..page_numbers.len()-1 {
                //If the pair of numbers doesn't exist in the page number rules then the sequence is invalid
                let pair = format!("{},{}", page_numbers[i], page_numbers[i+1]);
                if !page_number_rules.contains(&pair) {
                    not_valid = true;
                    break;
                }
            }

            //If the sequence is not valid, fix it and find the middle number and add it to the running total
            if not_valid {
                let number_set: HashSet<i32> = page_numbers.iter().copied().collect();
                let mut custom_graph: HashMap<i32, HashSet<i32>> = HashMap::new();
                let mut to_process = 0;
                let mut sorted: Vec<i32> = Vec::new();

                for num in &number_set {
                    let intersect: HashSet<i32> = number_set.intersection(graph.get(num).unwrap()).copied().collect();

                    if intersect.is_empty() {
                        //This is the last item in the sequence
                        sorted.push(*num);
                    }

                    if intersect.len() == 1 {
                        //The second to last number in the sequence only has one relationship
                        to_process = *num;
                    }

                    custom_graph.insert(*num, intersect);
                }

                //Now remove the irrelevant relationships, starting with the second to last as this will only have 1 relationship
                //Remove the one to which it points from all other lists
                //Find the next number that only points to this number
                loop {
                    //Build up our sorted list as we go...
                    sorted.push(to_process);

                    let to_remove: Option<i32>;

                    if let Some(set) = custom_graph.get(&to_process) {
                        if set.len() == 1 {
                            if let Some(value) = set.iter().next() {
                                to_remove = Some(*value);
                            } else {
                                panic!("Shouldn't get here");
                            }
                        } else {
                            panic!("Should only be 1 item in set but instead it contained: {:?} to_process: {to_process}", set);
                        }
                    } else {
                        panic!("Key should be in hashmap!");
                    }

                    if let Some(value) = to_remove {
                        custom_graph
                                    .iter_mut()
                                    .filter(|(key, _)| **key != to_process)
                                    .for_each(| (_, set) | {
                                        set.remove(&value);
                                    });
                        
                        //Find next item to process
                        if let Some((key, _)) = custom_graph
                                    .iter()
                                    .find(|(_, set)| set.contains(&to_process) && set.len() == 1) {
                            to_process = *key;
                        } else {
                            //We're finished
                           break;
                        }
                    }
                }

                //Technically the order of the sorted list is in reverse but since we only need the middle value it doesn't matter!
                running_total += sorted[sorted.len() / 2] as i64;
            }
        }
    }

    Ok(running_total)
}