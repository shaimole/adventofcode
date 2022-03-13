use util;
use std::path::Path;

pub fn solve1<P>(filename: P) -> i32 
where P: AsRef<Path>,{
    return 1;
}

pub fn solve2<P>(filename: P) -> i32 
where P: AsRef<Path>,{
    return 1;
}


fn read_data<P>(filename: P) -> Vec<Vec<String>> 
where P: AsRef<Path>, {
    let lines = util::read_lines(filename);
    return util::split_lines(lines," ");
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_testfile() {
        let result = super::read_data("././data/test");
        assert_eq!(result.len(),6);
    }

    // #[test]
    // fn it_should_solve_testdata_for_part_1() {
    //     assert_eq!(super::solve1("././data/test"),7);
    // }

    // #[test]    
    // fn it_should_solve_testdata_for_part_2() {
    //     assert_eq!(super::solve2("././data/test"),5);
    // }
}
