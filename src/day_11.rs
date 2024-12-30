//https://adventofcode.com/2024/day/11

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

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

#[derive(Copy, Clone, Debug)]
enum StoneProduced {
    Single(u64),
    Double((u64, u64)),
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 11 - Part 2:");
    
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

    //Part 2 we're blinking 75 times - this will take a lot longer if we use 
    //the simple solution from Part 1 and consume a lot of memory.
    //We only need the count of the stones produced not the arrangement.
    //Use a cache to remember which stones are produced

    let mut stone_cache: HashMap<u64, StoneProduced> = HashMap::new();
    let mut stone_count_cache: HashMap<(u64, i32), u64> = HashMap::new();
    let stop_at_blinks = 75;
    let mut stone_count: i64 = stones.len() as i64;

    //Start with the lowest number first, we'll be able to cache more
    stones.sort();

    for stone in stones {
        let produced: u64;
        if let Some(cache_output) = stone_count_cache.get(&(stone, stop_at_blinks)) {
            produced = *cache_output;
        } else {
            produced = get_stones_produced(&mut stone_cache, &mut stone_count_cache, stone, stop_at_blinks);
            stone_count_cache.insert((stone, stop_at_blinks), produced);
        }
        stone_count += produced as i64;
    }

    Ok(stone_count)
}

fn get_stones_produced(stone_cache: &mut HashMap<u64, StoneProduced>, stone_count_cache: &mut HashMap<(u64, i32), u64>, stone: u64, iteration: i32) -> u64 {
    if iteration == 0 {
        return 0
    }

    let stones_produced: u64;
    if let Some(cache_output) = stone_count_cache.get(&(stone, iteration)) {
        stones_produced = *cache_output;
    } else {
        let output: StoneProduced;

        if let Some(cache_output) = stone_cache.get(&stone) {
            output = *cache_output;
        } else {
            if stone == 0 {
                //Replace with 1
                output = StoneProduced::Single(1);
            } else {
                let stone_str = stone.to_string();
            
                if stone_str.len() % 2 == 0 {
                    //Even number of digits...split in two
                    let (left, right) = stone_str.split_at(stone_str.len() / 2);
                    output = StoneProduced::Double((left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()));
                } else {
                    //Replace stone value with its value multiplied by 2024
                    output = StoneProduced::Single(stone * 2024);
                }
            }
            //Store the result for this stone so we don't have to compute it each time
            stone_cache.insert(stone, output);
        }

        stones_produced = match output {
            StoneProduced::Single(single_stone) => get_stones_produced(stone_cache, stone_count_cache, single_stone, iteration - 1),
            StoneProduced::Double((left_stone, right_stone)) => {
                //We created an additional stone
                let mut produced = 1;
                produced += get_stones_produced(stone_cache, stone_count_cache, left_stone, iteration - 1);
                produced += get_stones_produced(stone_cache, stone_count_cache, right_stone, iteration - 1);
                produced
            },
        };

        //Store the stones produced for this stone at this iteration depth so we don't have to repeat the computation
        stone_count_cache.insert((stone, iteration), stones_produced);
    }

    stones_produced
}