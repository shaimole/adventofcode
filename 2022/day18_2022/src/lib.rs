use std::collections::HashSet;
use std::path::Path;

pub fn solve<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut front = HashSet::new();
    let mut side = HashSet::new();
    let mut top  = HashSet::new();
    front.len() + side.len() + top.len()
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
