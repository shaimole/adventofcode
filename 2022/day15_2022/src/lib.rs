use std::collections::HashSet;
use std::path::Path;

pub fn solve<P>(filename: P, y_pos: i64) -> u32
where
    P: AsRef<Path>,
{
    let coords: Vec<Vec<i64>> = common::read_lines(filename)
        .iter()
        .map(parse_coords)
        .collect();
    let mut impossible_positons: HashSet<(i64, i64)> = HashSet::new();
    for coord in &coords {
        let distance_to_nearest_becon =
            i64::abs(coord[2] - coord[0]) + i64::abs(coord[3] - coord[1]);
        let distance_to_target_y = i64::abs(y_pos - coord[1]);
        let impossible_position_amount = distance_to_nearest_becon - distance_to_target_y;
        if impossible_position_amount < 0 {
            continue;
        }

        for i in 0..=impossible_position_amount {
            impossible_positons.insert((coord[0] + i, y_pos));
            impossible_positons.insert((coord[0] - i, y_pos));
        }
    }
    for i in 0..coords.len() {
        impossible_positons.remove(&(coords[i][2], coords[i][3]));
    }
    impossible_positons.len() as u32
}

fn solve2<P>(filename: P, max_y: i64) -> i64
where
    P: AsRef<Path>,
{
    let coords: Vec<Vec<i64>> = common::read_lines(filename)
        .iter()
        .map(parse_coords)
        .collect();
    let mut impossible_positons: HashSet<(i64, i64)> = HashSet::new();
    for coord in &coords {
        let distance_to_nearest_becon =
            i64::abs(coord[2] - coord[0]) + i64::abs(coord[3] - coord[1]);
        for offset in 0..=distance_to_nearest_becon {
            let impossible_position_amount = distance_to_nearest_becon - offset;
            for i in 0..=impossible_position_amount {
                impossible_positons.insert((coord[0] + i, coord[1] + offset));
                impossible_positons.insert((coord[0] - i, coord[1] + offset));
                impossible_positons.insert((coord[0] + i, coord[1] - offset));
                impossible_positons.insert((coord[0] - i, coord[1] - offset));
            }
        }
    }
    for i in 0..=max_y {
        for j in 0..max_y {
            if impossible_positons.contains(&(i, j)) {
                continue;
            }
            if (i >= 0) && (i <= max_y) && (0 <= j) && (j <= max_y) {
                return i * 4000000 + j;
            }
        }
        // println!("");
    }
    // for pos in impossible_positons {}

    unreachable!();
}
fn parse_coords(line: &String) -> Vec<i64> {
    use regex::Regex;
    let re: Regex = Regex::new(r"-?\d+").unwrap();
    re.find_iter(line)
        .filter_map(|number| number.as_str().parse().ok())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_parse_positions() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(parse_coords(&lines[0]), vec![2, 18, -2, 15])
    }

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample", 10), 26)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample", 20), 56000011)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input", 2000000), 1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input", 10), 1)
    }
}
