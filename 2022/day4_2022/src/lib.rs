use common;
use std::path::Path;
use std::collections::HashMap;

pub fn solve1<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    0
}

pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    0
}

#[cfg(test)]
mod tests {

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
