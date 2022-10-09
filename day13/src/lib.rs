use common;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u16 
 where P: AsRef<Path>,{
  return 1;
}




pub fn solve2<P>(filename: P) -> u32 
where P: AsRef<Path>,{
    return 1;
}
 

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_parse_data_correctly() {   
    }
    
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(super::solve1("././data/sample"),17)
    }
    fn it_should_solve_part_1() {
    }

    #[test]
    fn it_should_solve_part_2() {
    }
}
