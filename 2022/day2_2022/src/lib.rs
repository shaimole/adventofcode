use common;
use std::path::Path;


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
    fn it_should_solve_part_1() {
        assert_eq!(super::solve1("./data/input"),1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(super::solve2("./data/input"),2)
    }

}