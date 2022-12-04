use common;
use std::path::Path;
use core::ops::Range;

pub fn solve1<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    0
}

pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    0
}

pub fn is_range_in_range(container: Range<u32>, target: Range<u32>) -> bool {
    container.contains(&target.start) && container.contains(&target.end)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_determine_if_one_range_contains_another() {
        assert_eq!(super::is_range_in_range(2..8,3..7),true);
        assert_eq!(super::is_range_in_range(2..3,3..7),false);
        assert_eq!(super::is_range_in_range(3..7,2..8),false);
    }
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(super::solve1("./data/sample"),2)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(super::solve2("./data/sample"),12)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(super::solve1("./data/input"),13484)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(super::solve2("./data/input"),13433)
    }

}
