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

    let mut hits = 0;
    for y in 0..=max_y {
        let mut x = 0;
        while x <= max_y {
            for coord in &coords {
                let sensor_range = i64::abs(coord[2] - coord[0]) + i64::abs(coord[3] - coord[1]);
                let current_distance_to_sensor = i64::abs(coord[0] - x) + i64::abs(coord[1] - y);
                if current_distance_to_sensor > sensor_range {
                    hits += 1;
                } else {
                    let currrent_y_distance_to_sensor = i64::abs(coord[1] - y);
                    let current_x_distance_to_sensor = coord[0] - x;
                    let impossible_position_amount = sensor_range - currrent_y_distance_to_sensor;
                    x += current_x_distance_to_sensor + impossible_position_amount;
                    // println!("sensor range {:?}, current x distance {:?}, self{:?}, sensor{:?},jumping {:?}", sensor_range, current_x_distance_to_sensor, (coord[0],coord[1]),(x,y) ,current_x_distance_to_sensor + impossible_position_amount);
                    break;
                }
            }
            if hits == coords.len() {
                return x * 4000000 + y;
            }
            hits = 0;
            x += 1;
        }
    }

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
        assert_eq!(solve("./data/input", 2000000), 5716881)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input", 4000000), 10852583132904)
    }
}
