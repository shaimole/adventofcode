use common;
use std::path::Path;
use core::ops::Range;

pub fn solve1<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    let pairs = common::split_lines(common::read_lines(filename),",");
    pairs.iter().map(to_ranges).map(score).sum()
}

pub fn to_ranges(ranges: &Vec<String>) -> Vec<Range<u32>> {
    ranges.iter().map(to_range).collect()
}





pub fn to_range(range: &String) -> Range<u32> {
   let vec: Vec<String> = range.split("-").map(|e| e.to_string()).collect();
    return vec[0].parse().unwrap()..vec[1].parse().unwrap();
}

fn score(ranges: Vec<Range<u32>>) -> u32 {
    if is_included_in_range(&ranges[0], &ranges[1]) {
        return 1
    }
    0
}

pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    0
}

pub fn is_range_in_range(container: &Range<u32>, target: &Range<u32>) -> bool {
    container.contains(&target.start) && container.contains(&target.end)
}
pub fn is_included_in_range(a: &Range<u32>, b: &Range<u32>) -> bool {
    is_range_in_range(a,b) || is_range_in_range(b,a)
}
#[cfg(test)]
mod tests {

    #[test]
    fn it_should_determine_if_one_range_contains_another() {
        assert_eq!(super::is_range_in_range(&(2..8),&(3..7)),true);
        assert_eq!(super::is_range_in_range(&(2..3),&(3..7)),false);
        assert_eq!(super::is_range_in_range(&(3..7),&(2..8)),false);
    }

    #[test]
    fn it_should_determine_if_any_range_contains_another() {
        assert_eq!(super::is_included_in_range(&(2..8),&(3..7)),true);
        assert_eq!(super::is_included_in_range(&(2..3),&(3..7)),false);
        assert_eq!(super::is_included_in_range(&(3..7),&(2..8)),true);
    }
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(super::solve1("./data/sample"),2)
    }

    #[test]
    fn it_should_convert_string_to_range() {
        assert_eq!(super::to_range(&"2-8".to_string()),2..8)
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
