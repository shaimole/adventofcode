use std::collections::HashMap;
use std::path::Path;

fn print(map: &HashMap<((i64, i64), u8)>, max: &(i64, i64)) {
    for y in (0..max.1).rev() {
        for x in 0..max.0 {
            if !map.contains_key(&(x, y)) {
                print!(" ");
            }
            if map.get(&(x, y)) == 0 {
                print!(".");
            }
            if map.get(&(x, y)) == 1 {
                print!("#");
            }
            if map.get(&(x, y)) == 2 {
                print!("0");
            }
        }
        println!("");
    }
}
pub fn solve<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    0
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
        assert_eq!(solve2("./data/sample") - 1)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), -1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), -1)
    }
}
