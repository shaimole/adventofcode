use std::collections::HashMap;
use std::path::Path;

fn print(map: &HashMap<(usize, usize), char>, max: &(usize, usize), pos: &(usize, usize)) {
    for y in 0..max.1 {
        for x in 0..max.0 {
            if pos == &(x, y) {
                print!("0");
                continue;
            }
            if !map.contains_key(&(x, y)) {
                print!(" ");
                continue;
            }
            print!("{}", map.get(&(x, y)).unwrap());
        }
        println!("");
    }
}

pub fn parse<P>(
    filename: P,
) -> (
    HashMap<(usize, usize), char>,
    (usize, usize),
    Vec<(usize, usize)>,
)
where
    P: AsRef<Path>,
{
    let mut max_x = 0;
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let lines = common::read_lines(filename);
    for y in 0..lines.len() {
        if lines[y].as_str() == "" {
            break;
        }
        for (x, c) in lines[y].chars().enumerate() {
            if c == ' ' {
                continue;
            }
            map.insert((x, y), c);
            max_x = std::cmp::max(x, max_x);
        }
    }
    let movement = parse_movement(&lines[lines.len() - 1]);
    (map, (max_x, lines.len() - 2), movement)
}

fn parse_movement(line: &String) -> Vec<(usize, usize)> {
    let parse = "R".to_owned() + &line.clone();
    use regex::Regex;
    let re: Regex = Regex::new(r"[A-Z][0-9]*").unwrap();
    re.find_iter(&parse)
        .map(|found| {
            let as_str = found.as_str();
            let direction: usize = match as_str.chars().nth(0).unwrap() {
                'R' => 1,
                'L' => 3,
                _ => unreachable!(),
            };

            let amount: usize = as_str[1..].parse().unwrap();
            return (direction, amount);
        })
        .collect()
}
pub fn solve<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let (map, max, movement) = parse(filename);
    let mut direction = 0;
    let directions: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut position = find_outer_edge(&map, &max, &0, &0, &direction);
    direction = 3;
    for (turn, steps) in movement.iter() {
        direction = (direction + turn) % directions.len();
        let velocity = directions[direction];
        for i in 0..*steps {
            //println!("velo {:?}",velocity);
            //println!("ditection  {:?}", direction);
            //println!("pos {:?}",position);
            //print(&map, &max, &position);
            //println!("");

            let mut target = (
                (position.0 as i64 + velocity.0) as usize,
                (position.1 as i64 + velocity.1) as usize,
            );
            if !map.contains_key(&(target)) {
                target = find_outer_edge(&map, &max, &position.0, &position.1, &direction);
            }
            if map.get(&target).unwrap() == &'#' {
                break;
            }
            position = target;
        }
    }
            println!("pos {:?}",position);
    1000 * (position.1+1) +  4 *  (1+position.0) + direction
}

fn find_outer_edge(
    map: &HashMap<(usize, usize), char>,
    max: &(usize, usize),
    x: &usize,
    y: &usize,
    direction: &usize,
) -> (usize, usize) {
    println!("{:?}", direction);
    if direction == &0 {
        for i in 0..=max.0 {
            if map.contains_key(&(i, *y)) {
                return (i, *y);
            }
        }
    }
    if direction == &1 {
        for i in 0..=max.1 {
            if map.contains_key(&(*x, i)) {
                return (*x, i);
            }
        }
    }
    if direction == &2 {
        for i in (0..=max.0).rev() {
            if map.contains_key(&(i, *y)) {
                return (i, *y);
            }
        }
    }
    if direction == &3 {
        for i in (0..=max.1).rev() {
            if map.contains_key(&(*x, i)) {
                return (*x, i);
            }
        }
    }
    unreachable!();
}

pub fn solve2<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 6032)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"), 2)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 2)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 2)
    }
}
