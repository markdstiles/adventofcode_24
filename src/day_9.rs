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
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    let mut checksum = 0_usize;

    if reader.read_line(&mut input).is_ok() {
        let mut disk_map: Vec<(usize, usize)> = input.chars()
            .enumerate()
            .map(|(idx, c)| (if idx % 2 == 0 { idx / 2 } else { 0 }, c.to_digit(10).unwrap() as usize))
            .collect();
        
        //Reverse the disk map so we can insert at the end of the vector rather than at the start - makes indexing easier to manage
        //So the back of the vector is actually the front of the disk
        disk_map.reverse();

        let mut rear_file_index: usize = 0;
        let mut last_file_id_moved = disk_map[0].0 + 1;
        loop {
            //Note - Length will be growing as we copy blocks to the front of the disk!
            if rear_file_index * 2 >= disk_map.len()-1 {
                //STOP! When we either reach the end of the vector
                break;
            }

            if disk_map[rear_file_index * 2].0 >= last_file_id_moved {
                rear_file_index += 1;
                //Skip any files that we've already processed
                continue;
            }

            let (file_id, file_size) = disk_map[rear_file_index * 2];
            //We could probably improve this by keeping track of where the space available starts
            let mut i_space_index: i32 = (disk_map.len() - 2) as i32;

            loop {
                if i_space_index < (rear_file_index * 2 + 1) as i32 {
                    //Stop searching for free space once we reach the file block, we won't move it
                    //Next file
                    rear_file_index += 1;
                    break
                }

                let space_index = i_space_index as usize;

                let space_available = disk_map[space_index].1;

                if space_available >= file_size {
                    let space_remaining = space_available - file_size;

                    //The file fits in the space...
                    //Space slot becomes 0 size
                    disk_map[space_index].1 = 0;
                    //Insert new file block
                    disk_map.insert(space_index, (file_id, file_size));
                    //Insert remaining space after the block
                    disk_map.insert(space_index, (0, space_remaining));

                    //Original file ID is set to 0 so it doesn't count when we sum up the checksum but we preserve the space it occupied
                    disk_map.remove(rear_file_index * 2);
                    //Collapse the space - unless we're the first item in the vector
                    if rear_file_index > 0 {
                        let next_space_size = disk_map[rear_file_index * 2 - 1].1;
                        disk_map[rear_file_index * 2].1 += file_size + next_space_size;
                        disk_map.remove(rear_file_index * 2 - 1);
                    } else {
                        //Remove the space at the end of the disk
                        disk_map.remove(rear_file_index * 2);
                    }
                    //No need to increment rear_file_index as we are now pointing at the next file
                    //since we shift the vector contents when we collapsed the space

                    last_file_id_moved = file_id;
                    break
                } 

                i_space_index -= 2;
            }
        }

        disk_map.reverse();

        let mut location = 0;
        for (file_id, size) in &disk_map {
            checksum += calc_checksum(*file_id, location..location+size);
            location += size;
        }
    }

    Ok(checksum as i64)
}