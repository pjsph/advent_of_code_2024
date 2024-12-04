use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    //part1();
    part2();
}

fn part1() {
    let file = File::open("src/day04/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut word_list: Vec<char> = vec![];

    let mut line_width: usize = 0;
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut chars: Vec<char> = line.chars().collect();
        if line_width == 0 {
            line_width = chars.len();
        }

        word_list.append(&mut chars);
    }

    let mut sum: u32 = 0;
    for i in 0..word_list.len() {
        for direction in CheckDirection::values() {
            if check_word(&word_list, "XMAS", line_width, i, direction).is_ok() {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn part2() {
    let file = File::open("src/day04/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut word_list: Vec<char> = vec![];

    let mut line_width: usize = 0;
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut chars: Vec<char> = line.chars().collect();
        if line_width == 0 {
            line_width = chars.len();
        }

        word_list.append(&mut chars);
    }

    let mut sum: u32 = 0;
    for i in 0..word_list.len() {
        if word_list[i] == 'A' {
            if !check_validity(word_list.len(), line_width, i) {
                continue;
            }

            let b = (check_word(&word_list, "MAS", line_width, i - line_width - 1, CheckDirection::DIAGONAL_DOWNRIGHT).is_ok() || check_word(&word_list, "MAS", line_width, i + line_width + 1, CheckDirection::DIAGONAL_UPLEFT).is_ok()) && (check_word(&word_list, "MAS", line_width, i + line_width - 1, CheckDirection::DIAGONAL_UPRIGHT).is_ok() || check_word(&word_list, "MAS", line_width, i - line_width + 1, CheckDirection::DIAGONAL_DOWNLEFT).is_ok());
            if b {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn check_validity(len: usize, line_width: usize, center: usize) -> bool {
    if (center % line_width) as i32 - 1 < 0 {
        return false;
    }
    if center % line_width + 1 >= line_width {
        return false;
    }
    if (center as i32) - (line_width as i32) < 0 {
        return false;
    }
    if center + line_width >= len {
        return false;
    }

    true
}

fn check_word(word_list: &Vec<char>, word: &str, line_width: usize, index: usize, direction: CheckDirection) -> Result<(), ()> {
    let word_chars: Vec<char> = word.chars().collect();
    match direction {
        CheckDirection::HORIZONTAL_RIGHT => {
            // RIGHT
            for i in 0..word_chars.len() {
                if index % line_width + i >= line_width {
                    return Err(());
                }
                if word_list[index + i] != word_chars[i] {
                    return Err(());
                }
            }
        },
        CheckDirection::HORIZONTAL_LEFT => {
            // LEFT
            for i in 0..word_chars.len() {
                if (index % line_width) as i32 - (i as i32) < 0 {
                    return Err(());
                }
                if word_list[index - i] != word_chars[i] {
                    return Err(());
                }
            }
        },
        CheckDirection::VERTICAL_UP => {
            // UP
            for i in 0..word_chars.len() {
                if index as i32 - ((i * line_width) as i32) < 0 {
                    return Err(());
                }
                if word_list[index - i * line_width] != word_chars[i] {
                    return Err(());
                }
            }
        },
        CheckDirection::VERTICAL_DOWN => {
            // DOWN
            for i in 0..word_chars.len() {
                if index as i32 + ((i * line_width) as i32) >= word_list.len() as i32 {
                    return Err(());
                }
                if word_list[index + i * line_width] != word_chars[i] {
                    return Err(());
                }
            }
        },
        CheckDirection::DIAGONAL_UPRIGHT => {
            // FROM DOWN LEFT TO UP RIGHT
            for i in 0..word_chars.len() {
                if index as i32 - ((i * line_width) as i32) < 0 || index % line_width + i >= line_width {
                    return Err(());
                }
                if word_list[index - i * line_width + i] != word_chars[i] {
                    return Err(());
                }
            }
        },
        CheckDirection::DIAGONAL_DOWNRIGHT => {
            // FROM UP LEFT TO DOWN RIGHT
            for i in 0..word_chars.len() {
                if index as i32 + ((i * line_width) as i32) >= word_list.len() as i32 || index % line_width + i >= line_width {
                    return Err(());
                }
                if word_list[index + i * line_width + i] != word_chars[i] {
                    return Err(());
                }
            }
        },
        CheckDirection::DIAGONAL_UPLEFT => {
            // FROM DOWN RIGHT TO UP LEFT 
            for i in 0..word_chars.len() {
                if index as i32 - ((i * line_width) as i32) < 0 || (index % line_width) as i32 - (i as i32) < 0 {
                    return Err(());
                }
                if word_list[index - i * line_width - i] != word_chars[i] {
                    return Err(());
                }
            }
        },
        CheckDirection::DIAGONAL_DOWNLEFT => {
            // FROM UP RIGHT TO DOWN LEFT 
            for i in 0..word_chars.len() {
                if index as i32 + ((i * line_width) as i32) >= word_list.len() as i32 || (index % line_width) as i32 - (i as i32) < 0 {
                    return Err(());
                }
                if word_list[index + i * line_width - i] != word_chars[i] {
                    return Err(());
                }
            }
        }
    };

    Ok(())
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum CheckDirection {
    HORIZONTAL_RIGHT,
    HORIZONTAL_LEFT,
    VERTICAL_UP,
    VERTICAL_DOWN,
    DIAGONAL_UPRIGHT,
    DIAGONAL_UPLEFT,
    DIAGONAL_DOWNRIGHT,
    DIAGONAL_DOWNLEFT,
}

impl CheckDirection {
    pub fn values() -> Vec<Self> {
        vec![Self::HORIZONTAL_RIGHT, Self::HORIZONTAL_LEFT, Self::VERTICAL_UP, Self::VERTICAL_DOWN, Self::DIAGONAL_UPRIGHT, Self::DIAGONAL_UPLEFT, Self::DIAGONAL_DOWNRIGHT, Self::DIAGONAL_DOWNLEFT]
    }
}
