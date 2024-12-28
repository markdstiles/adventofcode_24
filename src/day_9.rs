//https://adventofcode.com/2024/day/9

use std::{fs::File, io::{BufRead, BufReader}, ops::Range};

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 9 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day9.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let mut reader = BufReader::new(file);
    let mut input = String::new();

    //File is 19,999 characters
    //First character is a file size in blocks, second character is size of space, this alternates repeatedly
    //We have 10,000 file block sizes and 9,999 space block sizes
    //Aim is to move the blocks from the rear into space at the front, working back to front
    //Calculating a checksum as we go based on (file id * file pos) + (file id * file pos) ... + (file id * file pos)
    //We should be able to do this in one pass working simulatenously from front and rear, creeping inwards until the cursors would overlap

    let mut checksum = 0_usize;

    if let Ok(len) = reader.read_line(&mut input) {
        let disk_map: Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        
        let mut front_file_id = 0_usize;
        let mut rear_file_id = len / 2;
        let mut space_location = 0_usize;
        let mut space_remaining = 0_usize;
        let mut file_size_remaining = 0_usize;

        while front_file_id <= rear_file_id {
            let front_file_size = disk_map[front_file_id * 2];
            let rear_file_size = disk_map[rear_file_id * 2];
            let space_available = disk_map[(front_file_id * 2) + 1];
 
            if space_remaining == 0 {
                checksum += calc_checksum(front_file_id, space_location..space_location + front_file_size);
                space_location += front_file_size;
            }

            //Scenarios:
            //Rear file is bigger than the space - we'll have no space remaining and file blocks left to allocate
            //Rear file is smaller than the space - we'll have space remaining and no file blocks left to allocate
            //Rear file fits completely in space (unlikely!) - we'll have no space remaining or file blocks left to allocate
            
            let space_allocated: usize = if file_size_remaining > 0 {
                if file_size_remaining > space_available {
                    file_size_remaining -= space_available;
                    space_available
                } else {
                    let allocated = file_size_remaining;
                    space_remaining = space_available - file_size_remaining;
                    file_size_remaining = 0;
                    allocated
                }
            } else if space_remaining > 0 {
                if rear_file_size > space_remaining {
                    let allocated = space_remaining;
                    file_size_remaining = rear_file_size - space_remaining;
                    space_remaining = 0;
                    allocated
                } else {
                    space_remaining -= rear_file_size;
                    rear_file_size
                }
            } else /* file_size_remaining and space_remanining are both 0 */ if rear_file_size > space_available {
                file_size_remaining = rear_file_size - space_available;
                space_available
            } else {
                space_remaining = space_available - rear_file_size;
                rear_file_size
            };

            checksum += calc_checksum(rear_file_id, space_location..space_location + space_allocated);
            space_location += space_allocated; 

            //Once the free space at the front of the disk is consumed we can move onto the next file/space block
            if space_remaining == 0 {
                front_file_id += 1;
            }

            //Process next rear file once previous file is completely re-allocated
            if file_size_remaining == 0 {
                rear_file_id -= 1;
            }
        }
    }

    Ok(checksum as i64)
}

fn calc_checksum(file_id: usize, block_range: Range<usize>) -> usize {
    block_range.fold(0, |sum, idx| sum + (idx * file_id))
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 9 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day9.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let _reader = BufReader::new(file);

    Ok(0)
}