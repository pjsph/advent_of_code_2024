use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn main() {
    //part1();
    part2();
}

fn part1() {
    let file = File::open("src/day06/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut chars: Vec<char> = vec![];

    let mut line_width: usize = 0;
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut line_chars: Vec<char> = line.chars().collect();
        if line_width == 0 {
            line_width = line_chars.len();
        }

        chars.append(&mut line_chars);
    }

    let mut start_index: usize = 0;
    let mut start_direction = Direction::FACING_NORTH;
    for i in 0..chars.len() {
        if chars[i] == '^' {
            start_index = i;
            start_direction = Direction::FACING_NORTH;
            break;
        } else if chars[i] == '>' {
            start_index = i;
            start_direction = Direction::FACING_EAST;
            break;
        } else if chars[i] == 'v' {
            start_index = i;
            start_direction = Direction::FACING_SOUTH;
            break;
        } else if chars[i] == '<' {
            start_index = i;
            start_direction = Direction::FACING_WEST;
            break;
        }
    }

    let path = compute_path(&chars, line_width, start_index, start_direction).unwrap();

    println!("{}", path.len());
}

fn part2() {
    let file = File::open("src/day06/inputs.txt").expect("Error while reading input file.");
    let reader = BufReader::new(file);

    let mut chars: Vec<char> = vec![];

    let mut line_width: usize = 0;
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut line_chars: Vec<char> = line.chars().collect();
        if line_width == 0 {
            line_width = line_chars.len();
        }

        chars.append(&mut line_chars);
    }

    let mut start_index: usize = 0;
    let mut start_direction = Direction::FACING_NORTH;
    for i in 0..chars.len() {
        if chars[i] == '^' {
            start_index = i;
            start_direction = Direction::FACING_NORTH;
            break;
        } else if chars[i] == '>' {
            start_index = i;
            start_direction = Direction::FACING_EAST;
            break;
        } else if chars[i] == 'v' {
            start_index = i;
            start_direction = Direction::FACING_SOUTH;
            break;
        } else if chars[i] == '<' {
            start_index = i;
            start_direction = Direction::FACING_WEST;
            break;
        }
    }

    let path = compute_path(&chars, line_width, start_index, start_direction).unwrap();

    let mut sum: usize = 0;
    let mut new_chars = chars.clone();
    for i in 1..path.len() {
        print!(".");
        std::io::stdout().flush();
        new_chars[path[i].0] = '#';

        if !compute_path(&new_chars, line_width, start_index, start_direction).is_ok() {
            sum += 1;
        }

        new_chars[path[i].0] = chars[path[i].0];
    }
    println!("");

    println!("{}", sum);
}

fn compute_path(
    chars: &Vec<char>,
    line_width: usize,
    index: usize,
    direction: Direction,
) -> Result<Vec<(usize, Vec<Direction>)>, ()> {
    let mut curr_index = index;
    let mut curr_direction = direction;
    let mut path: Vec<(usize, Vec<Direction>)> = vec![(curr_index, vec![curr_direction])];
    loop {
        if let Ok(action) = step(chars, line_width, &mut curr_index, &mut curr_direction) {
            match action {
                Action::MOVE => {
                    let mut found = false;
                    for el in &mut path {
                        if el.0 == curr_index {
                            if el.1.contains(&curr_direction) {
                                // we are looping
                                return Err(());
                            }
                            el.1.push(curr_direction);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        path.push((curr_index, vec![curr_direction]));
                    }
                }
                Action::TURN => {
                    let mut found = false;
                    for el in &mut path {
                        if el.0 == curr_index {
                            el.1.push(curr_direction);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        path.push((curr_index, vec![curr_direction]));
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok(path)
}

fn step(
    chars: &Vec<char>,
    line_width: usize,
    index: &mut usize,
    direction: &mut Direction,
) -> Result<Action, ()> {
    match move_forward(chars, line_width, *index, *direction) {
        Ok(new_index) => {
            *index = new_index;
            Ok(Action::MOVE)
        }
        Err(e) => match e {
            MoveError::OUT_OF_BOUND => Err(()),
            MoveError::OBSTACLE => {
                *direction = turn_right(*direction);
                Ok(Action::TURN)
            }
        },
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::FACING_NORTH => Direction::FACING_EAST,
        Direction::FACING_EAST => Direction::FACING_SOUTH,
        Direction::FACING_SOUTH => Direction::FACING_WEST,
        Direction::FACING_WEST => Direction::FACING_NORTH,
    }
}

fn move_forward(
    chars: &Vec<char>,
    line_width: usize,
    index: usize,
    direction: Direction,
) -> Result<usize, MoveError> {
    let mut next_index: usize = 0;
    match direction {
        Direction::FACING_NORTH => {
            let ni = index as i32 - line_width as i32;
            if ni < 0 {
                return Err(MoveError::OUT_OF_BOUND);
            }
            next_index = ni as usize;
        }
        Direction::FACING_SOUTH => {
            let ni = index + line_width;
            if ni >= chars.len() {
                return Err(MoveError::OUT_OF_BOUND);
            }
            next_index = ni;
        }
        Direction::FACING_EAST => {
            if (index % line_width) + 1 >= line_width {
                return Err(MoveError::OUT_OF_BOUND);
            }
            next_index = index + 1;
        }
        Direction::FACING_WEST => {
            if (index % line_width) as i32 - 1 < 0 {
                return Err(MoveError::OUT_OF_BOUND);
            }
            next_index = index - 1;
        }
    }

    if chars[next_index] == '#' {
        //println!("Obstacle at {}", next_index);
        return Err(MoveError::OBSTACLE);
    }

    Ok(next_index)
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    FACING_NORTH,
    FACING_EAST,
    FACING_SOUTH,
    FACING_WEST,
}

#[repr(u8)]
enum Action {
    MOVE,
    TURN,
}

enum MoveError {
    OUT_OF_BOUND,
    OBSTACLE,
}
