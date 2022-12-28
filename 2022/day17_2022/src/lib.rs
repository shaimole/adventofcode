use std::path::Path;
use std::collections::HashSet;


pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>
{
    0
}


pub fn solve2<P>(filename: P) -> u32
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
        assert_eq!(solve("./data/sample"), 31)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 29)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 504)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 500)
    }
}
