use util;
use std::path::Path;

pub fn solve1<P>(filename: P, steps: u16) -> u16 
where P: AsRef<Path>,{
   return 1;
}

pub fn solve2<P>(filename: P, steps: u16) -> u16 
where P: AsRef<Path>,{
    return 1;
}



#[cfg(test)]
mod tests {
    fn it_should_read_testdata_as_2_dim_array() {
        let expected = [
            [5,4,8,3,1,4,3,2,2,3],
            [2,7,4,5,8,5,4,7,1,1],
            [5,2,6,4,5,5,6,1,7,3],
            [6,1,4,1,3,3,6,1,4,6],
            [6,3,5,7,3,8,5,4,7,8],
            [4,1,6,7,5,2,4,6,4,5],
            [2,1,7,6,8,4,1,7,2,1],
            [6,8,8,2,8,8,1,1,3,4],
            [4,8,4,6,8,4,8,5,5,4],
            [5,2,8,3,7,5,1,5,2,6]
        ];
        assert_eq!(super::read_data("././data/test"),expected);
    }
    #[test]
    fn it_should_solve_testdata_for_part_1() {
        assert_eq!(super::solve1("././data/test",10),204);
        assert_eq!(super::solve1("././data/test",10),1656);
    }
}
 
