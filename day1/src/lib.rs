use common;
use std::path::Path;

// maybe have a look at slices, e.g. smt like data.as_slice()[1..] + zip
// that gives you pairs and then you can filter and count all in one iterator expression!
pub fn solve1<P>(filename: P) -> u32 
where P: AsRef<Path>,{
    let data = parse_data(filename);
    return get_maximum_consecutive_depth_increase(data)
}

fn parse_data<P>(filename: P) -> Vec<u32> 
where P: AsRef<Path>, {
    let lines = common::read_lines(filename);
    return common::lines_to_int(lines).iter().map(|e| *e as u32).collect();
}

fn get_maximum_consecutive_depth_increase(data: Vec<u32>) -> u32 {
    let touples = data.iter().zip(data.iter().skip(1));
    return touples.filter(|(prev_depth, current_depth)| prev_depth < current_depth)
        .collect::<Vec<(&u32,&u32)>>().len() as u32;
}

pub fn solve2<P>(filename: P) -> u32 
where P: AsRef<Path>,{
    let data = parse_data(filename);
    let grouped = group_depths_in_triplets(data);
    return get_maximum_consecutive_depth_increase(grouped);
}

fn group_depths_in_triplets(data: Vec<u32>) -> Vec<u32> {
    let mut grouped = vec![]; 
    for i in 2..data.len() {
        grouped.push(data[i-2] + data[i-1] + data[i]);
    }
    return grouped;
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_testfile() {
        let result = super::parse_data("././data/test");
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

       #[test]
    fn it_should_solve_part_1() {
        assert_eq!(super::solve1("././data/sample1"),1154);
    }

    #[test]    
    fn it_should_solve_part_2() {
        assert_eq!(super::solve2("././data/sample1"),1127);
    }
}
