//https://adventofcode.com/2024/day/17

struct ComputerState {
    register_a: u32,
    register_b: u32,
    register_c: u32,
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

        let combo_operand: u32 = match operand {
            0..=3 => operand as u32,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => 7, //*** Reserved ***
        };

        if debug_flag {
            println!("IP:{ip} A:{register_a} B:{register_b} C:{register_c}");
            println!("OC:{op_code} OP:{operand} COP: {combo_operand} JC: {jump_count}");
        }

        match op_code {
            0 => {
                //adv - Division on register A
                register_a /= 2_u32.pow(combo_operand);
            },
            1 => {
                //bxl - Bitwise XOR of literal
                register_b ^= operand as u32;
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
                register_b = register_a / 2_u32.pow(combo_operand);
            },
            7 => {
                //cdv - Division on register A but store result in register C
                register_c = register_a / 2_u32.pow(combo_operand);
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

    Ok(0)
}