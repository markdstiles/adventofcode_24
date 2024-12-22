//https://adventofcode.com/2024/day/4

use std::{fs::File, io::{BufRead, BufReader}};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 4 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day4.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut word_search: Vec<String> = Vec::new();

    for line in reader.lines() {
        word_search.push(line?.clone());
    }

    // Word              S  S  S
    // Search             A A A
    //                     MMM
    //Searching for XMAS SAMXMAS   
    //                     MMM
    //                    A A A
    //                   S  S  S

    let height: usize = word_search.len();
    let width: usize = word_search[0].len();
    let mut word_count = 0;

    //Start at the top-left, if we find X then we scan in all directions for 'XMAS'
    let mut y: usize = 0;
    while y < height {
        let mut x: usize = 0;
        while x < width {

            if word_search[y].chars().nth(x).unwrap() == 'X' {

                if x > 2 {
                    //Scan west
                    if word_search[y][x-3..x] == *"SAM" {
                        word_count += 1;
                    }

                    if y > 2 {
                        //Scan diagonally north-west
                        if word_search[y-1].chars().nth(x-1).unwrap() == 'M'
                        && word_search[y-2].chars().nth(x-2).unwrap() == 'A'
                        && word_search[y-3].chars().nth(x-3).unwrap() == 'S' {
                            word_count += 1;
                        }
                    }

                    if y < height - 3 {
                        //Scan diagonally south-west
                        if word_search[y+1].chars().nth(x-1).unwrap() == 'M'
                        && word_search[y+2].chars().nth(x-2).unwrap() == 'A'
                        && word_search[y+3].chars().nth(x-3).unwrap() == 'S' {
                            word_count += 1;
                        }
                    }
                }

                if x < width - 3 {
                    //Scan east
                    if word_search[y][x..x+4] == *"XMAS" {
                        word_count += 1;
                    }

                    if y > 2 { 
                        //Scan diagonally north-east
                        if word_search[y-1].chars().nth(x+1).unwrap() == 'M'
                        && word_search[y-2].chars().nth(x+2).unwrap() == 'A'
                        && word_search[y-3].chars().nth(x+3).unwrap() == 'S' {
                            word_count += 1;
                        }
                    }

                    if y < height - 3 { 
                        //Scan diagonally south-east
                        if word_search[y+1].chars().nth(x+1).unwrap() == 'M'
                        && word_search[y+2].chars().nth(x+2).unwrap() == 'A'
                        && word_search[y+3].chars().nth(x+3).unwrap() == 'S' {
                            word_count += 1;
                        }
                    }
                }

                if y > 2 {
                    //Scan north
                    if word_search[y-1].chars().nth(x).unwrap() == 'M'
                    && word_search[y-2].chars().nth(x).unwrap() == 'A'
                    && word_search[y-3].chars().nth(x).unwrap() == 'S' {
                        word_count += 1;
                    }
                }

                if y < height - 3 {
                    //Scan south
                    if word_search[y+1].chars().nth(x).unwrap() == 'M'
                    && word_search[y+2].chars().nth(x).unwrap() == 'A'
                    && word_search[y+3].chars().nth(x).unwrap() == 'S' {
                        word_count += 1;
                    }
                }
            }

            x += 1;
        }
        y += 1;
    }

    Ok(word_count)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 4 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day4.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut word_search: Vec<String> = Vec::new();

    for line in reader.lines() {
        word_search.push(line?.clone());
    }

    // Word              
    // Search            
    //                         S M  M M
    //Searching for Cross-MAS - A    A  
    //                         S M  S S

    let height: usize = word_search.len();
    let width: usize = word_search[0].len();
    let mut mas_count = 0;

    //Start at the top-left, we can skip the first row/col
    //Look for the central A letter, then scan NE,NW,SE,SW for S or M
    let mut y: usize = 1;
    while y < height-1 {
        let mut x: usize = 1;
        while x < width-1 {
            mas_count += if word_search[y].chars().nth(x).unwrap() == 'A'                                                    //Center is A
                        && (word_search[y-1].chars().nth(x+1).unwrap() == 'S' || word_search[y-1].chars().nth(x+1).unwrap() == 'M') //NE is either S or M
                        && (word_search[y-1].chars().nth(x-1).unwrap() == 'S' || word_search[y-1].chars().nth(x-1).unwrap() == 'M') //NW
                        && (word_search[y+1].chars().nth(x+1).unwrap() == 'S' || word_search[y+1].chars().nth(x+1).unwrap() == 'M') //SE
                        && (word_search[y+1].chars().nth(x-1).unwrap() == 'S' || word_search[y+1].chars().nth(x-1).unwrap() == 'M') //SW
                        && word_search[y-1].chars().nth(x+1).unwrap() != word_search[y+1].chars().nth(x-1).unwrap() //NE != SW (diagonals must not be the same, one is M and the other must be S)
                        && word_search[y-1].chars().nth(x-1).unwrap() != word_search[y+1].chars().nth(x+1).unwrap() //NW != SE
                        { 1 } else { 0 };   //Add 1 to count if we get a match
            x += 1;
        }
        y += 1;
    }

    Ok(mas_count)
}