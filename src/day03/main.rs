use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    //part1();
    part2();
}

fn part1() {
    let file = File::open("src/day03/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut input: Vec<char> = vec![];

    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut chars: Vec<char> = line.chars().collect();
        input.append(&mut chars);
    }

    let mut curr_index: usize = 0;

    let mut count = 0;
    while curr_index < input.len() {
        if try_consume_str(&input, &mut curr_index, "mul(") {
            if let Ok(a) = try_consume_alpha(&input, &mut curr_index) {
                if try_consume_str(&input, &mut curr_index, ",") {
                    if let Ok(b) = try_consume_alpha(&input, &mut curr_index) {
                        if try_consume_str(&input, &mut curr_index, ")") {
                            count += a*b;
                        }
                    }
                }
            }
        }
    }
    println!("{count}");
}

fn part2() {
    let file = File::open("src/day03/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut input: Vec<char> = vec![];

    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut chars: Vec<char> = line.chars().collect();
        input.append(&mut chars);
    }

    let mut curr_index: usize = 0;

    let mut enabled = true;
    let mut count = 0;
    while curr_index < input.len() {
        let mut start_index = curr_index;
        if try_consume_str(&input, &mut curr_index, "do()") {
            enabled = true;
        } else {
            curr_index = start_index;
        }
        if try_consume_str(&input, &mut curr_index, "don't()") {
            enabled = false;
        } else {
            curr_index = start_index;
        }
        if try_consume_str(&input, &mut curr_index, "mul(") {
            if let Ok(a) = try_consume_alpha(&input, &mut curr_index) {
                if try_consume_str(&input, &mut curr_index, ",") {
                    if let Ok(b) = try_consume_alpha(&input, &mut curr_index) {
                        if try_consume_str(&input, &mut curr_index, ")") {
                            if enabled {
                                count += a*b;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{count}");
}

fn try_consume_str(input: &Vec<char>, curr_index: &mut usize, token: &str) -> bool {
    let start_index = *curr_index;
    let chars: Vec<char> = token.chars().collect();
    while *curr_index - start_index < chars.len() && peek(input, *curr_index) == chars[*curr_index - start_index] {
        advance(input, curr_index);
    }
    if start_index == *curr_index { // skip
        *curr_index += 1;
        return false;
    }
    start_index + chars.len() == *curr_index
}

fn try_consume_alpha(input: &Vec<char>, curr_index: &mut usize) -> Result<u32, ()> { 
    let start_index = *curr_index;

    while peek(input, *curr_index).is_digit(10) {
        advance(input, curr_index);
    }

    if start_index == *curr_index {
        *curr_index += 1;
        return Err(());
    }

    let parse = &input[start_index..*curr_index].into_iter().map(|c| c.to_digit(10)).try_fold(0, |ans, i| i.map(|i| ans * 10 + i));

    if parse.is_none() {
        return Err(());
    }

    Ok(parse.unwrap())
}

fn advance(input: &Vec<char>, curr_index: &mut usize) -> char {
    let c = input[*curr_index];
    *curr_index += 1;
    c
}

fn peek(input: &Vec<char>, curr_index: usize) -> char {
    input[curr_index]
}
