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
pub mod input_utils;

use colored::Colorize;

fn main() -> anyhow::Result<()> {
    
    if let Ok(answer) = day_14::do_part1() {
        println!("{} {}", "Part 1 answer:".green(), answer.to_string().blue().bold());
    }

    if let Ok(answer) = day_14::do_part2() {
        println!("{} {}", "Part 2 answer:".green(), answer.to_string().blue().bold());
    }

    Ok(())
}
