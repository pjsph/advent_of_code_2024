use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn main() {
    //part1();
    part2();
}

fn part1() {
    let file = File::open("src/day05/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];

    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        lines.push(line);
    }

    let split_index = lines.iter().position(|el| el == "").unwrap();
    let (rules_str, updates_str) = (&lines[0..split_index], &lines[split_index + 1..]);

    let mut rules = HashMap::new();
    for rule_str in rules_str {
        let parsed: Vec<u32> = rule_str
            .split("|")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let entry = rules.entry(parsed[0]).or_insert(vec![parsed[1]]);
        entry.push(parsed[1]);
    }

    let updates: Vec<Vec<u32>> = updates_str
        .iter()
        .map(|update| {
            update
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut good_updates: Vec<Vec<u32>> = vec![];
    for update in updates {
        if is_update_good(&update, &rules) {
            good_updates.push(update.clone());
        }
    }

    let mut sum: u32 = 0;
    for update in good_updates {
        sum += update[update.len() / 2];
    }

    println!("{}", sum);
}

fn part2() {
    let file = File::open("src/day05/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];

    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        lines.push(line);
    }

    let split_index = lines.iter().position(|el| el == "").unwrap();
    let (rules_str, updates_str) = (&lines[0..split_index], &lines[split_index + 1..]);

    let mut rules = HashMap::new();
    for rule_str in rules_str {
        let parsed: Vec<u32> = rule_str
            .split("|")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let entry = rules.entry(parsed[0]).or_insert(vec![parsed[1]]);
        entry.push(parsed[1]);
    }

    let updates: Vec<Vec<u32>> = updates_str
        .iter()
        .map(|update| {
            update
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut bad_updates: Vec<Vec<u32>> = vec![];
    for update in updates {
        if !is_update_good(&update, &rules) {
            bad_updates.push(update.clone());
        }
    }

    for i in 0..bad_updates.len() {
        fix_update(&mut bad_updates[i], &rules);
    }

    let mut sum: u32 = 0;
    for update in bad_updates {
        sum += update[update.len() / 2];
    }

    println!("{}", sum);
}

fn fix_update(update: &mut Vec<u32>, rules: &HashMap<u32, Vec<u32>>) {
    while !is_update_good(update, rules) {
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                if rules.get(&update[i]).is_none() || !rules[&update[i]].contains(&update[j]) {
                    let tmp = update[i];
                    update[i] = update[j];
                    update[j] = tmp;
                }
            }
        }
    }
}

fn is_update_good(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if !rules[&update[i]].contains(&update[j]) {
                return false;
            }
        }
    }
    true
}
