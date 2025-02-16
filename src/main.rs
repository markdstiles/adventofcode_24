pub mod input_utils;
pub mod misc_types;
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20; //TODO
pub mod day_21; //TODO
pub mod day_22; //TODO
pub mod day_23; //TODO
pub mod day_24; //TODO
pub mod day_25; //TODO

use colored::Colorize;

fn main() -> anyhow::Result<()> {
    
    if let Ok(answer) = day_19::do_part1() {
        println!("{} {}", "Part 1 answer:".green(), answer.to_string().blue().bold());
    }

    if let Ok(answer) = day_19::do_part2() {
        println!("{} {}", "Part 2 answer:".green(), answer.to_string().blue().bold());
    }

    Ok(())
}
