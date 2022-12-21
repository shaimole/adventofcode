use std::path::Path;

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}

fn solve<P>(filename: P) -> u32
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
        assert_eq!(solve("./data/sample"), 13)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 140)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 1)
    }
}
