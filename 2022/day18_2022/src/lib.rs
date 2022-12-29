use std::collections::HashSet;
use std::path::Path;

pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut cube: HashSet<(i16, i16, i16)> = HashSet::new();
    let mut size = 0;
    common::read_lines(filename)
        .iter()
        .map(|line| line.split(",").collect())
        .for_each(|line: Vec<&str>| {
            let x: i16 = line[0].parse().unwrap();
            let y: i16 = line[1].parse().unwrap();
            let z: i16 = line[2].parse().unwrap();
            cube.insert((x, y, z));
            size = std::cmp::max(size,std::cmp::max(std::cmp::max(x,y),z))
        });

    let mut open_sides = 0;
    for z in (0..=size).rev() {
        for y in (0..=size).rev() {
            for x in 0..=size {
                if cube.contains(&(x, y, z)) {
                    if !cube.contains(&(x + 1, y, z)) {
                        open_sides += 1;
                    }
                    if !cube.contains(&(x - 1, y, z)) {
                        open_sides += 1;
                    }
                    if !cube.contains(&(x, y + 1, z)) {
                        open_sides += 1;
                    }
                    if !cube.contains(&(x, y - 1, z)) {
                        open_sides += 1;
                    }
                    if !cube.contains(&(x, y, z + 1)) {
                        open_sides += 1;
                    }
                    if !cube.contains(&(x, y, z - 1)) {
                        open_sides += 1;
                    }
                }
            }
        }
    }
    open_sides
}

fn solve2<P>(filename: P) -> i64
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
        assert_eq!(solve("./data/sample"), 64)
    }

    #[test]
    fn it_should_solve_poc() {
        assert_eq!(solve("./data/poc"), 10)
    }
    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 64)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 64)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 64)
    }
}
