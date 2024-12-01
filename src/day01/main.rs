use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

pub fn main() {
    //part1();
    part2();
}

fn part1() {
    let file = File::open("src/day01/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut numbers1: Vec<i32> = vec![];
    let mut numbers2: Vec<i32> = vec![];
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let s = line.split(" ").collect::<Vec<&str>>();
        numbers1.push(s[0].parse::<i32>().expect("Error while parsing number."));
        numbers2.push(s[3].parse::<i32>().expect("Error while parsing number."));
    }

    numbers1.sort();
    numbers2.sort();

    let mut sum = 0i32;
    for i in 0..numbers1.len() {
        sum += (numbers1[i] - numbers2[i]).abs();
    }

    println!("{sum}");
}

fn part2() {
    let file = File::open("src/day01/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut numbers1: Vec<i32> = vec![];
    let mut numbers2: Vec<i32> = vec![];
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let s = line.split(" ").collect::<Vec<&str>>();
        numbers1.push(s[0].parse::<i32>().expect("Error while parsing number."));
        numbers2.push(s[3].parse::<i32>().expect("Error while parsing number."));
    }

    let mut map: HashMap<i32, i32> = HashMap::new();

    for el in numbers2 {
        let entry = map.entry(el).or_insert(0);
        *entry += 1;
    }

    let mut similarity = 0i32;
    for i in 0..numbers1.len() {
        similarity += numbers1[i] * map.get(&numbers1[i]).unwrap_or(&0);
    }

    println!("{similarity}");

}
