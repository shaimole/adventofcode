use std::collections::HashSet;
use std::path::Path;

pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut cube: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut size = 0;
    common::read_lines(filename)
        .iter()
        .map(|line| line.split(",").collect())
        .for_each(|line: Vec<&str>| {
            let x: i32 = line[0].parse().unwrap();
            let y: i32 = line[1].parse().unwrap();
            let z: i32 = line[2].parse().unwrap();
            cube.insert((x, y, z));
            size = std::cmp::max(size, std::cmp::max(std::cmp::max(x, y), z))
        });

    let mut open_sides = 0;
    cube.iter().for_each(|point| {
        get_adjecent(point, &size, &-1).iter().for_each(|a| {
            if !cube.contains(a) {
                open_sides += 1;
            }
        });
    });
    open_sides
}

fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut cube: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut size: i32 = 0;
    common::read_lines(filename)
        .iter()
        .map(|line| line.split(",").collect())
        .for_each(|line: Vec<&str>| {
            let x: i32 = line[0].parse().unwrap();
            let y: i32 = line[1].parse().unwrap();
            let z: i32 = line[2].parse().unwrap();
            cube.insert((x, y, z));
            size = std::cmp::max(size, std::cmp::max(std::cmp::max(x, y), z))
        });
    println!("{:?}", get_adjecent(&(0, 0, 0), &size, &0));
    let mut open_sides = 0;
    open_sides
}

fn get_adjecent(point: &(i32, i32, i32), size: &i32, lower_limit: &i32) -> Vec<(i32, i32, i32)> {
    let mut directions: Vec<(i32, i32, i32)> = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    directions
        .iter()
        .map(|direction| {
            (
                point.0 + direction.0,
                point.1 + direction.1,
                point.2 + direction.2,
            )
        })
        .filter(|point| !is_out_of_bounds(point, size, lower_limit))
        .collect()
}

fn is_out_of_bounds(point: &(i32, i32, i32), size: &i32, lower_limit: &i32) -> bool {
    for coordinate in [point.0, point.1, point.2].iter() {
        if *coordinate > size + 1 || coordinate < lower_limit {
            return true;
        }
    }
    false
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
        assert_eq!(solve("./data/input"), 4504)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 64)
    }
}
