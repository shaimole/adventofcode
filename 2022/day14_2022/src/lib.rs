use std::collections::HashSet;
use std::path::Path;

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let (max_y, mut map) = as_map(filename);
    for i in 0..10000 {
        map.insert((i, max_y + 2));
    }
    let mut score = 0;
    let mut sand_pos = (500, 0);
    while true {
        if map.contains(&sand_pos) {
            return score;
        }
        if !map.contains(&(sand_pos.0, sand_pos.1 + 1)) {
            sand_pos.1 += 1;
        } else if !map.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos.0 -= 1;
            sand_pos.1 += 1;
        } else if !map.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos.0 += 1;
            sand_pos.1 += 1;
        } else {
            map.insert(sand_pos);
            score += 1;
            sand_pos = (500, 0);
        }
    }
    0
}

fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let (max_y, mut map) = as_map(filename);
    let mut score = 0;
    let mut sand_pos = (500, 0);
    for step in 1..1000000 {
        if !map.contains(&(sand_pos.0, sand_pos.1 + 1)) {
            sand_pos.1 += 1;
        } else if !map.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos.0 -= 1;
            sand_pos.1 += 1;
        } else if !map.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos.0 += 1;
            sand_pos.1 += 1;
        } else {
            map.insert(sand_pos);
            score += 1;
            sand_pos = (500, 0);
        }
        println!("{:?}", sand_pos);
        for i in 0..=9 {
            for j in 494..=503 {
                if map.contains(&(j, i)) {
                    print!("x")
                } else {
                    print!(".")
                }
            }
            println!("");
        }
        if sand_pos.1 > max_y {
            return score;
        }
    }
    0
}
fn as_map<P>(filename: P) -> (u32, HashSet<(u32, u32)>)
where
    P: AsRef<Path>,
{
    let mut max_y = 0;
    let lines = common::split_lines(common::read_lines(filename), " -> ");
    let mut map = HashSet::new();
    for cave in lines {
        let split: Vec<&str> = cave[0].split(",").collect();
        let mut x: u32 = split[0].clone().parse().unwrap();
        let mut y: u32 = split[1].clone().parse().unwrap();
        for i in 1..cave.len() {
            let split: Vec<&str> = cave[i].split(",").collect();
            let new_x = split[0].clone().parse().unwrap();
            let new_y = split[1].clone().parse().unwrap();
            println!("{:?}", vec![(x, new_x), (y, new_y)]);
            if x == new_x {
                for j in std::cmp::min(y, new_y)..=std::cmp::max(y, new_y) {
                    map.insert((x, j));
                }
            }

            if y == new_y {
                for j in std::cmp::min(x, new_x)..=std::cmp::max(x, new_x) {
                    map.insert((j, y));
                }
            }
            max_y = std::cmp::max(max_y, y);
            x = new_x;
            y = new_y;
        }
    }
    (max_y, map)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 24)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 93)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 1298)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 25585)
    }
}
