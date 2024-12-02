use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    //part1();
    part2();
}

fn part1() {
    let file = File::open("src/day02/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;
    'outer: for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let numbers: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().expect("Error while parsing number.")).collect();

        if check_line(&numbers).is_ok() {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn part2() {
    let file = File::open("src/day02/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;
    'outer: for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let numbers: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().expect("Error while parsing number.")).collect();

        match check_line(&numbers) {
            Ok(()) => {
                sum += 1;
            },
            Err(err_index) => {
                for i in 0..3usize {
                    if err_index as i32 - i as i32 >= 0 {
                        let mut corrected_numbers = numbers.clone();
                        corrected_numbers.remove(err_index-i);
                        match check_line(&corrected_numbers) {
                            Ok(()) => {
                                sum += 1;
                                break;
                            },
                            Err(_) => {}
                        }
                    }
                }
            }
        }
    }

    println!("{}", sum);
}

fn check_line(numbers: &Vec<i32>) -> Result<(), (usize)>  {
    let mut diff: i32 = 0;
    for i in 1..numbers.len() {
        let new_diff = numbers[i] - numbers[i-1];
        if new_diff.abs() < 1 || new_diff.abs() > 3 {
            // check if between 1 and 3
            return Err(i);
        }

        if (diff < 0 && new_diff >= 0) || (diff > 0 && new_diff <= 0) {
            // different sign (incr then decr or opposite way)
            return Err(i);
        }

        diff = new_diff;
    }
    Ok(())
}
