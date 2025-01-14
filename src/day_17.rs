//https://adventofcode.com/2024/day/17

use std::collections::HashSet;

struct ComputerState {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    ip: usize,
    memory: Vec<u8>,
    debug_flag: bool,
}

fn run_program(state: ComputerState) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];

    let ComputerState { 
        mut register_a, 
        mut register_b, 
        mut register_c, 
        mut ip, 
        memory,
        debug_flag,
    } = state;

    let mut jump_count = 0;

    loop {
        if ip >= memory.len() {
            break;
        }

        if jump_count > 10 {
            //Deadlock protection!
            //if we get more than 10 consecutive jumps we're probably stuck in a loop!
            println!("Deadlock detected - halting program");
            break;
        }

        let op_code = memory[ip];
        let operand = memory[ip+1];
        //Amount to increment the instruction pointer by, this can be changed by the jnz instruction
        let mut inc: usize = 2;

        let combo_operand: usize = match operand {
            0..=3 => operand as usize,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => 7, //*** Reserved ***
        };

        if debug_flag {
            println!("IP:{ip} A:{register_a} B:{register_b} ({}) C:{register_c}", register_b % 8);
            //println!("OC:{op_code} OP:{operand} COP: {combo_operand} JC: {jump_count}");
        }

        match op_code {
            0 => {
                //adv - Division on register A
                register_a /= 2_usize.pow(combo_operand as u32);
            },
            1 => {
                //bxl - Bitwise XOR of literal
                register_b ^= operand as usize;
            },
            2 => {
                //bst - Modulo
                register_b = combo_operand % 8;
            },
            3 => {
                //jnz - Jump not zero
                jump_count += 1;
                if register_a != 0 {
                    ip = operand as usize;
                    inc = 0;
                }
            },
            4 => {
                //bxc - Bitwise XOR of register C with register B
                register_b ^= register_c;
            },
            5 => {
                //out - Output value mod 8
                output.push((combo_operand % 8) as u8);
            },
            6 => {
                //bdv - Division on register A but store result in register B
                register_b = register_a / 2_usize.pow(combo_operand as u32);
            },
            7 => {
                //cdv - Division on register A but store result in register C
                register_c = register_a / 2_usize.pow(combo_operand as u32);
            },
            _ => break,
        }

        if op_code != 3 {
            jump_count = 0;
        }

        ip += inc;
    }

    if debug_flag {
        println!("HALTING... Final state:");
        println!("IP:{ip} A:{register_a} B:{register_b} C:{register_c}");
    }

    output
}

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 17 - Part 1:");
    
    /*
    Register A: 59397658
    Register B: 0
    Register C: 0

    Program: 2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0
    */

    let _test_program_state = ComputerState {
        register_a: 729,
        register_b: 0,
        register_c: 0,
        memory: vec![0,1,5,4,3,0],
        ip: 0,
        debug_flag: false,
    };

    let state = ComputerState {
        register_a: 59397658,
        register_b: 0,
        register_c: 0,
        memory: vec![2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0],
        ip: 0,
        debug_flag: false,
    };

    let output: Vec<u8> = run_program(state);
    
    if !output.is_empty() {
        let out: String = output.iter()
            .map(|i| i.to_string())
            .reduce(|curr, nxt| curr + "," + &nxt).unwrap();
        println!("{out}");
    }

    Ok(0)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 17 - Part 2:");

    let target = [2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0];
    
    //This is brute force approach and will run for a long time but gets to the answer within 3-5mins
    //202366627359274.25

    if let Some(input) = brute_force(&target) {
        return Ok(input as i64);
    }

    Ok(0)
}

fn test_input(input: usize) -> Vec<usize> {
    let mut a = input;
    let mut output: Vec<usize> = vec![];

    //This is the equivalent of what the program achieves, the question is...
    //can it be reversed to find the input value that matches the program parameters.
    while a != 0 {
        //B = (A / 2^((A MOD 8) XOR -/+ 1)) XOR ((A MOD 8) XOR 1) XOR +/- 4
        let exp = (a % 8) ^ 1;
        let b = ((a / 2_usize.pow(exp as u32)) ^ exp) ^ 4;
        output.push(b % 8);
        a /= 8;
    }        

    output
}

fn brute_force(output: &[u8]) -> Option<usize> {
    //Tries to find the original input given an output for this function:

    /*
    while a != 0 {
        //B = (A / 2^((A MOD 8) XOR -/+ 1)) XOR ((A MOD 8) XOR 1) XOR +/- 4
        let exp = (a % 8) ^ 1;
        let b = ((a / 2_usize.pow(exp as u32)) ^ exp) ^ 4;
        output.push(b % 8);
        a /= 8;
    }
    */

    //Starting with the last digit, try and find values for A by multiply it by 8 to reverse the 
    //function of the program. Through a bit of iterative testing it looks like A becomes fractional 
    //through one or more of the division steps and then truncated making it hard to find.
    //Simply searching for a whole number for A would take too long or we may miss it completely

    let output: Vec<usize> = output.iter().map(|&i| i as usize).collect();
    let mut test_values: HashSet<(String, usize)> = HashSet::new();

    //The last digit can only be generated by a number between 0 and 8
    for a in 0..8 {
        if let Some(b) = test_input(a).iter().last() {
            if *b == output[output.len() - 1] {
                test_values.insert(((a as f64).to_string(), output.len() - 2));
            }
        }
    }

    let mut count = 0;
    let mut tested = 0;

    //Keep track of the lowest match
    let mut lowest_match_found: Option<usize> = None;
    //To avoid retesting we remember matches we've already found
    let mut matched: HashSet<usize> = HashSet::new();

    while let Some((a_str, next_to_match)) = test_values.iter().last().cloned() {
        let a: f64 = a_str.parse().unwrap();

        //Skip if we already matched this fraction
        if matched.contains(&(a as usize)) {
            continue;
        }

        //Original is fractional so hunt through fractions of power of 8 values
        for x in 0..100 {
            let n = x as f64 / 100_f64;
            let new_a = (a + n) * 8_f64;
            let test_output = test_input(new_a as usize);
            let test_index = output.len() - test_output.len();
            tested += 1;

            if test_output.len() <= output.len() && test_output[..] == output[test_index..] {
                count += 1;
                //println!("{}/{} a:{a} new a:{new_a} -> {:?}<>{:?}", test_output.len(), output.len(), test_output, &output[test_index..]);
                if test_output == output {
                    //it all matched!
                    matched.insert(new_a as usize);

                    //Do we have a new lowest value?
                    if let Some(curr_value) = lowest_match_found {
                        if curr_value > new_a as usize {
                            lowest_match_found = Some(new_a as usize);
                            //Print it out so we don't have to wait an hour for the result!
                            println!("{new_a} ({next_to_match})");
                        }
                    } else {
                        println!("{new_a}");
                        lowest_match_found = Some(new_a as usize);
                    }

                    break;
                } else if next_to_match != 0 {
                    //Keep going if there are digits in the output string to process
                    let next = (new_a.to_string(), next_to_match - 1);
                    
                    //My theory is the number will be 16 digits long based on a bit of testing with shorter numbers
                    if (new_a as usize).to_string().len() > 16 {
                        //If number is longer than 16 then stop
                        return lowest_match_found;
                    }

                    //If we've not tested the combination before then add it to the list to test
                    if !test_values.contains(&next) {
                        test_values.insert(next);
                    }
                } 
            }
        }

        test_values.remove(&(a_str, next_to_match));
    }

    println!("Matches: {count} Tested: {tested}");

    lowest_match_found
}