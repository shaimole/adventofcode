use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

const ADJECENT: &'static [(i64, i64); 8] = &[
    (1, 0),
    (0, -1),
    (-1, 0),
    (0, 1),
];

fn debug(elfs: &HashSet<(i64, i64)>, min_y: i64, min_x: i64, max_x: i64, max_y: i64) {
    for y in (min_y..=max_y).rev() {
        // print!("{y}");
        for x in min_x..=max_x {
            if !elfs.contains(&(x, y)) {
                print!(".")
            } else {
                print!("#");
            }
        }
        println!("");
    }
}
pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}

fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}

fn parse<P>(filename: P) -> HashSet<(i64, i64)>
where
    P: AsRef<Path>,
{
    let mut map = HashSet::new();
    common::read_lines(filename)
        .iter()
        .rev()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    map.insert((x as i64, y as i64));
                }
            });
        });
    map
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 18)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 1)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 1)
    }
}
