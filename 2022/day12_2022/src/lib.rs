use std::path::Path;

pub fn solve<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    // parse input as matrix/vec of vec str
    // find starting position as in (i,j) 
    // init steps with 0
    // init visited as an empty vec

    // start traveling pass current steps 0 , visited, and starting position as current cell
    // if starting pos equals end return steps;
    // look at adjecent cells
    // for each cell determine if cell was 1. not visited before 2. is elevation of at most 1 more as current cell
        // if true
            //  increment steps by one
            // clone visited and append cell
            // start traveling pass current steps +1  , visited clone , and this cell beining looked at
            // returning value will be 

    // return steps
   0 
}

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    1
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
        // assert_eq!(solve2("./data/sample"), 1)
    }

    #[test]
    fn it_should_solve_part_1() {
        // assert_eq!(solve("./data/input"), 15020)
    }

    #[test]
    fn it_should_solve_part_2() {
        // assert_eq!(solve2("./data/input"), 1)
    }
}
