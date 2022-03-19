use common;
use std::path::Path;

// maybe have a look at slices, e.g. smt like data.as_slice()[1..] + zip
// that gives you pairs and then you can filter and count all in one iterator expression!
pub fn solve1<P>(filename: P) -> u32 
where P: AsRef<Path>,{
    let data = read_data(filename);
    let touples = data.iter().zip(data.iter().skip(1));
    return touples.filter(|(a,b)| a < b).collect::<Vec<(&u32,&u32)>>().len() as u32;
}

pub fn solve2<P>(filename: P) -> u32 
where P: AsRef<Path>,{
    let data = read_data(filename);
    let mut previous_depth = 0;
    let mut rise_counter = 0;
    for i in 3..data.len() {
        let depth = data[i-2] + data[i-1] + data[i];
         if depth > previous_depth {
            rise_counter = rise_counter + 1;
        }
        previous_depth = depth;
    }
    return rise_counter;
}


fn read_data<P>(filename: P) -> Vec<u32> 
where P: AsRef<Path>, {
    let lines = common::read_lines(filename);
    return common::lines_to_int(lines).iter().map(|e| *e as u32).collect();
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_testfile() {
        let result = super::read_data("././data/test");
        assert_eq!(result.len(),10);
    }

    #[test]
    fn it_should_solve_testdata_for_part_1() {
        assert_eq!(super::solve1("././data/test"),7);
    }

    #[test]    
    fn it_should_solve_testdata_for_part_2() {
        assert_eq!(super::solve2("././data/test"),5);
    }
}
