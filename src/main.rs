pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

use colored::Colorize;

fn main() -> anyhow::Result<()> {
    
    if let Ok(answer) = day_4::do_part1() {
        println!("{} {}", "Part 1 answer:".green(), answer.to_string().blue().bold());
    }

    if let Ok(answer) = day_4::do_part2() {
        println!("{} {}", "Part 2 answer:".green(), answer.to_string().blue().bold());
    }

    Ok(())
}