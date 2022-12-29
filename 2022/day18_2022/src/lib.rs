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
            size = std::cmp::max(size, std::cmp::max(std::cmp::max(x, y), z))
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

fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut cube: HashSet<(u16, u16, u16)> = HashSet::new();
    let mut size: u16 = 0;
    common::read_lines(filename)
        .iter()
        .map(|line| line.split(",").collect())
        .for_each(|line: Vec<&str>| {
            let x: u16 = line[0].parse().unwrap();
            let y: u16 = line[1].parse().unwrap();
            let z: u16 = line[2].parse().unwrap();
            cube.insert((x, y, z));
            size = std::cmp::max(size, std::cmp::max(std::cmp::max(x, y), z))
        });

    let mut open_sides = 0;
    for z in (0..=size).rev() {
        for y in (0..=size).rev() {
            for x in 0..=size {
                if cube.contains(&(x, y, z)) {
                    if !cube.contains(&(x + 1, y, z)) {
                        let air = &(x + 1, y, z);
                        println!("{:?}",detect_inner_air(air, &cube, &size));
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
            println!("");
        }
        println!("");
    }

    open_sides
}

fn detect_inner_air(air: &(u16, u16, u16), cube: &HashSet<(u16, u16, u16)>, size: &u16) -> i8 {
    if &air.0 == size || &air.1 == size || &air.2 == size {
        return -1;
    }
    let mut sum = 0;
    let x = air.0;
    let y = air.1;
    let z = air.2;
    if !cube.contains(&(x + 1, y, z)) {
        sum += detect_inner_air(&(x + 1, y, z), cube, size);
    }
    if !cube.contains(&(x - 1, y, z)) {
        sum += detect_inner_air(&(x - 1, y, z), cube, size);
    }
    if !cube.contains(&(x, y + 1, z)) {
        sum += detect_inner_air(&(x, y + 1, z), cube, size);
    }
    if !cube.contains(&(x, y - 1, z)) {
        sum += detect_inner_air(&(x, y - 1, z), cube, size);
    }
    if !cube.contains(&(x, y, z + 1)) {
        sum += detect_inner_air(&(x, y, z), cube, size);
    }
    if !cube.contains(&(x, y, z - 1)) {
        sum += detect_inner_air(&(x, y, z), cube, size);
    }
    sum
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
        assert_eq!(solve2("./data/sample"), 58)
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
