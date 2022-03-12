use util;
use std::path::Path;

pub fn solve<P>(filename: P) -> i32 
where P: AsRef<Path>,{
    let data = get_data(filename);
    let mut previous_depth = 0;
    let mut rise_counter = -1;
    for depth in data {
        if depth > previous_depth {
            rise_counter = rise_counter + 1;
        }
        previous_depth = depth;
    }
    return rise_counter;
}

fn get_data<P>(filename: P) -> Vec<i32> 
where P: AsRef<Path>, {
    let lines = util::read_lines(filename);
    return util::lines_to_int(lines);
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_testfile() {
        let result = super::get_data("././data/test");
        assert_eq!(result.len(),10);
    }
    #[test]
    fn it_should_solve_testdata() {
        assert_eq!(super::solve("././data/test"),7);
    }
}
