use std::collections::HashSet;

pub fn solve(datastream: &[char], sequence_len: usize) -> usize {
    sequence_len
        + datastream
            .windows(sequence_len)
            .position(is_all_unique)
            .unwrap()
}

fn is_all_unique(set: &[char]) -> bool {
    set.iter().collect::<HashSet<&char>>().len() == set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        let datastream_1: Vec<char> = common::read_lines("./data/sample")[0].chars().collect();
        assert_eq!(solve(&datastream_1, 4), 5);
        let datastream_2: Vec<char> = common::read_lines("./data/sample")[1].chars().collect();
        assert_eq!(solve(&datastream_2, 4), 6);
        let datastream_3: Vec<char> = common::read_lines("./data/sample")[2].chars().collect();
        assert_eq!(solve(&datastream_3, 4), 10);
        let datastream_4: Vec<char> = common::read_lines("./data/sample")[3].chars().collect();
        assert_eq!(solve(&datastream_4, 4), 11);
    }

    #[test]
    fn it_should_solve_sample_part2() {
        let datastream_1: Vec<char> = common::read_lines("./data/sample2")[0].chars().collect();
        assert_eq!(solve(&datastream_1, 14), 19);
        let datastream_2: Vec<char> = common::read_lines("./data/sample2")[1].chars().collect();
        assert_eq!(solve(&datastream_2, 14), 23);
        let datastream_3: Vec<char> = common::read_lines("./data/sample2")[2].chars().collect();
        assert_eq!(solve(&datastream_3, 14), 23);
        let datastream_4: Vec<char> = common::read_lines("./data/sample2")[3].chars().collect();
        assert_eq!(solve(&datastream_4, 14), 29);
    }
    #[test]
    fn it_should_solve_part_1() {
        let datastream: Vec<char> = common::read_lines("./data/input")[0].chars().collect();
        assert_eq!(solve(&datastream, 4), 1876)
    }

    #[test]
    fn it_should_solve_part_2() {
        let datastream: Vec<char> = common::read_lines("./data/input")[0].chars().collect();
        assert_eq!(solve(&datastream, 14), 2202)
    }
}
