//https://adventofcode.com/2024/day/19

use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

//Non-Deterministic Finite Automata
#[derive(Debug)]
struct Nfa<T> {
    initial_state: usize,
    accept_state: usize,
    transitions: HashMap<(usize, Option<T>), HashSet<usize>>,
    current_states: HashSet<usize>,
    state_count: usize,
    accepted_state_hit_count: usize,
}

impl<T> Nfa<T> 
    where T : Clone + Copy + Eq + std::hash::Hash + std::fmt::Debug
{
    fn new() -> Nfa<T> {
        let initial_state = 0;
        let accept_state = 1;
        let mut current_states = HashSet::new();
        current_states.insert(initial_state);

        let mut transitions: HashMap<(usize, Option<T>), HashSet<usize>> = HashMap::new();
        //State transition from accept to initial state with empty state
        // - this allows the states to loop back to the start from the accept state.
        transitions.insert((1, None), current_states.clone());

        Nfa {
            initial_state,
            accept_state,
            transitions,
            current_states,
            state_count: 2,     //2 states - Initial & accept state
            accepted_state_hit_count: 0,
        }
    }

    fn reset(&mut self) {
        self.current_states.clear();
        self.current_states.insert(self.initial_state);
        self.accepted_state_hit_count = 0;
    }

    fn apply_state(&mut self, new_state: T) {
        let mut new_current_states: HashSet<usize> = HashSet::new();

        for index in &self.current_states {
            let next_state = (*index, Some(new_state));
            if let Some(next_states) = self.transitions.get(&next_state) {
                new_current_states = new_current_states.union(next_states).copied().collect();
            }
        }

        self.current_states = new_current_states;

        //Can we also traverse the empty state from the new current state?
        for index in self.current_states.clone() {
            let empty_state = (index, None);
            if let Some(next_states) = self.transitions.get(&empty_state) {
                self.current_states = self.current_states.union(next_states).copied().collect();
            }
        }
    }

    fn apply_states(&mut self, new_states: &[T]) {
        new_states.iter().for_each(|state| self.apply_state(*state));
    }

    fn is_acceptable(&self) -> bool {
        let accept_set = HashSet::from([self.accept_state]);
        self.current_states.intersection(&accept_set).any(|_| true)
    }

    fn recognise_states(&mut self, valid_transitions: &[T]) {
        let mut current_index = self.initial_state;

        for i in 0..valid_transitions.len() - 1 {
            let value = valid_transitions[i];
            let state = (current_index, Some(value));
            let mut next_index = self.state_count;
            let mut add_next_index = true;

            if let Some(next_indices) = self.transitions.get(&state) {
                //Does the transition map already contain the next state?
                for next in next_indices {
                    let next_state = (*next, Some(valid_transitions[i + 1]));
                    if self.transitions.contains_key(&next_state) {
                        next_index = *next;
                        add_next_index = false;
                        break;
                    }
                }
            }

            if add_next_index {
                self.transitions.entry(state)
                    .and_modify(|n| {n.insert(next_index);})
                    .or_insert(HashSet::from([next_index]));

                self.state_count += 1;
            }

            current_index = next_index;
        }

        //Last state is in the accept state
        let last_state = (current_index, Some(valid_transitions[valid_transitions.len()-1]));
        self.transitions.entry(last_state).and_modify(|n| {n.insert(self.accept_state);}).or_insert(HashSet::from([self.accept_state]));
    }
}

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 19 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day19.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
    let mut is_first_line = true;
    
    let mut valid_patterns: Vec<String> = vec![];
    let mut patterns_to_test: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        if is_first_line {
            valid_patterns = line.split(", ").map(|s| s.to_string()).collect();
            is_first_line = false;
        } else if !line.is_empty() {
            patterns_to_test.push(line);
        }
    }

    //Use a non-deterministic finite automata to test the patterns are valid
    let mut nfa: Nfa<char> = Nfa::new();
    valid_patterns.sort_by_key(|a| a.len());

    //First we must define the valid states for the automata, only add unique state transitions
    for valid_pattern in valid_patterns {
        let states: Vec<char> = valid_pattern.chars().collect();

        //Test whether the pattern is already recognised by the finite automata
        nfa.apply_states(&states);
        if !nfa.is_acceptable() {
            //Its not recognised...
            nfa.recognise_states(&states);
        }
        //Reset the finite automata
        nfa.reset();
    }

    //Now test the patterns
    let mut valid_count = 0;
    for pattern in patterns_to_test {
        nfa.apply_states(&pattern.chars().collect::<Vec<char>>());

        if nfa.is_acceptable() {
           valid_count += 1; 
        }
        //Reset the finite automata
        nfa.reset();
    }

    Ok(valid_count)
}

fn calculate_combinations<'a>(pattern: &'a str, valid_patterns: &Vec<&str>, history: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(count) = history.get(pattern) {
        *count
    } else {
        let mut count = 0;  //If there are no valid combinations, the result will be zero as this value won't change
        for valid_pattern in valid_patterns {
            if let Some(next) = pattern.strip_prefix(valid_pattern) {
                //Sum the valid combinations, history saves up from repeatedly deriving each branch of the tree
                count += calculate_combinations(next, valid_patterns, history);
                history.entry(pattern)
                    .and_modify(|c| *c = count)
                    .or_insert(count);
            }
        }
        count
    }
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 19 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day19.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
    let mut is_first_line = true;
    
    let mut valid_patterns: Vec<String> = vec![];
    let mut patterns_to_test: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        if is_first_line {
            valid_patterns = line.split(", ").map(|s| s.to_string()).collect();
            is_first_line = false;
        } else if !line.is_empty() {
            patterns_to_test.push(line);
        }
    }

    let mut total_combinations = 0;

    //Using a recursive approach for part 2
    for pattern in patterns_to_test.iter().map(|p| p.as_str()) {
        let mut history: HashMap<&str, usize> = HashMap::new();
        //The recursive function will repeatedly strip each valid pattern as a prefix
        //when we get to the empty slice, we've found a full valid pattern combination
        history.insert("", 1);  
        total_combinations += calculate_combinations(pattern, &valid_patterns.iter().map(|p| p.as_str()).collect(), &mut history);
    }

    Ok(total_combinations as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nfa_single_pattern_valid() {
        let mut nfa: Nfa<char> = Nfa::new();
        let valid_states: Vec<char> = vec!['a', 'b', 'c'];
        nfa.recognise_states(&valid_states);

        let test: Vec<char> = vec!['a', 'b', 'c'];
        nfa.apply_states(&test);

        assert!(nfa.is_acceptable());
    }

    #[test]
    fn nfa_single_pattern_invalid() {
        let mut nfa: Nfa<char> = Nfa::new();
        let valid_states: Vec<char> = vec!['a', 'b', 'c'];
        nfa.recognise_states(&valid_states);

        let test: Vec<char> = vec!['a', 'b', 'd'];
        nfa.apply_states(&test);

        assert!(!nfa.is_acceptable());
    }

    #[test]
    fn nfa_multi_pattern_valid() {
        let mut nfa: Nfa<char> = Nfa::new();
        let valid_states1: Vec<char> = vec!['a'];
        let valid_states2: Vec<char> = vec!['f'];
        let valid_states3: Vec<char> = vec!['a', 'b', 'c'];
        nfa.recognise_states(&valid_states1);
        nfa.recognise_states(&valid_states2);
        nfa.recognise_states(&valid_states3);

        let test: Vec<char> = vec!['a', 'f', 'a', 'a', 'b', 'c', 'f'];
        nfa.apply_states(&test);

        assert!(nfa.is_acceptable());
    }

    #[test]
    fn nfa_multi_pattern_invalid() {
        let mut nfa: Nfa<char> = Nfa::new();
        let valid_states1: Vec<char> = vec!['a'];
        let valid_states2: Vec<char> = vec!['a', 'b', 'c'];
        nfa.recognise_states(&valid_states1);
        nfa.recognise_states(&valid_states2);

        let test: Vec<char> = vec!['a', 'a', 'a', 'b', 'a', 'c'];
        nfa.apply_states(&test);

        assert!(!nfa.is_acceptable());
    }

    #[test]
    fn nfa_aoc_part1_test_case() {
        let mut nfa: Nfa<char> = Nfa::new();

        nfa.recognise_states(&['r']);
        nfa.recognise_states(&['b']);
        nfa.recognise_states(&['g']);
        nfa.recognise_states(&"wr".chars().collect::<Vec<char>>());
        nfa.recognise_states(&"bwu".chars().collect::<Vec<char>>());

        nfa.apply_states(&"brwrr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());

        nfa.reset();
        nfa.apply_states(&"bggr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());

        nfa.reset();
        nfa.apply_states(&"gbbr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());

        nfa.reset();
        nfa.apply_states(&"rrbgbr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());

        nfa.reset();
        nfa.apply_states(&"ubwu".chars().collect::<Vec<char>>());
        assert!(!nfa.is_acceptable());

        nfa.reset();
        nfa.apply_states(&"bwurrg".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());

        nfa.reset();
        nfa.apply_states(&"brgr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());

        nfa.reset();
        nfa.apply_states(&"bbrgwb".chars().collect::<Vec<char>>());
        assert!(!nfa.is_acceptable());
    }

    #[test]
    fn nfa_aoc_part2_test_case() {
        let mut nfa: Nfa<char> = Nfa::new();

        nfa.recognise_states(&['r']);
        nfa.recognise_states(&['b']);
        nfa.recognise_states(&['g']);
        nfa.recognise_states(&"wr".chars().collect::<Vec<char>>());
        nfa.recognise_states(&"rb".chars().collect::<Vec<char>>());
        nfa.recognise_states(&"gb".chars().collect::<Vec<char>>());
        nfa.recognise_states(&"br".chars().collect::<Vec<char>>());
        nfa.recognise_states(&"bwu".chars().collect::<Vec<char>>());

        nfa.apply_states(&"brwrr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());
        let mut history: HashMap<&str, usize> = HashMap::new();
        history.insert("", 1);
        assert_eq!(2, calculate_combinations("brwrr", &vec!["r", "b", "g", "wr", "rb", "gb", "br", "bwu"], &mut history));

        nfa.reset();
        nfa.apply_states(&"bggr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());
        let mut history: HashMap<&str, usize> = HashMap::new();
        history.insert("", 1);
        assert_eq!(1, calculate_combinations("bggr", &vec!["r", "b", "g", "wr", "rb", "gb", "br", "bwu"], &mut history));

        nfa.reset();
        nfa.apply_states(&"gbbr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());
        let mut history: HashMap<&str, usize> = HashMap::new();
        history.insert("", 1);
        assert_eq!(4, calculate_combinations("gbbr", &vec!["r", "b", "g", "wr", "rb", "gb", "br", "bwu"], &mut history));

        nfa.reset();
        nfa.apply_states(&"rrbgbr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());
        let mut history: HashMap<&str, usize> = HashMap::new();
        history.insert("", 1);
        assert_eq!(6, calculate_combinations("rrbgbr", &vec!["r", "b", "g", "wr", "rb", "gb", "br", "bwu"], &mut history));

        nfa.reset();
        nfa.apply_states(&"ubwu".chars().collect::<Vec<char>>());
        assert!(!nfa.is_acceptable());

        nfa.reset();
        nfa.apply_states(&"bwurrg".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());
        let mut history: HashMap<&str, usize> = HashMap::new();
        history.insert("", 1);
        assert_eq!(1, calculate_combinations("bwurrg", &vec!["r", "b", "g", "wr", "rb", "gb", "br", "bwu"], &mut history));

        nfa.reset();
        nfa.apply_states(&"brgr".chars().collect::<Vec<char>>());
        assert!(nfa.is_acceptable());
        let mut history: HashMap<&str, usize> = HashMap::new();
        history.insert("", 1);
        assert_eq!(2, calculate_combinations("brgr", &vec!["r", "b", "g", "wr", "rb", "gb", "br", "bwu"], &mut history));

        nfa.reset();
        nfa.apply_states(&"bbrgwb".chars().collect::<Vec<char>>());
        assert!(!nfa.is_acceptable());
    }
}