use std::collections::Hashmap;
use std::path::Path;
fn parse<P>(filename: P) -> (HashMap<String, i128>, Vec<(String, String, String)>)
where
    P: AsRef<Path>,
{
    let mut known: HashMap<String, i128> = HashMap::new();
    let mut unknown: Vec<String, String, String> = vec![];
    common::read_lines(filename).iter().for_each(|line| {});
    (known, unknown)
}

pub fn solve<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    0
}

pub fn solve2<P>(filename: P) -> i128
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
        assert_eq!(solve("./data/sample"), 152)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"), -1)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), -1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), -1)
    }
}
