use util;
use std::path::Path;

pub fn solve1<P>(filename: P) -> i32 
where P: AsRef<Path>,{
    let data = read_data(filename);
    let mut current_depth = 0;
    let mut current_distance = 0;
    for movement in data {
        let direction = &movement[0];
        let distance = movement[1].parse::<i32>().unwrap();
        if distance == 0 {
            continue;
        }
        if direction == "forward" {
            current_distance = current_distance + distance;
        }
        if direction == "up" {
            current_depth =  current_depth - distance;
        }
        if direction == "down" {
            current_depth = current_depth + distance;
        }
    }
    return current_depth * current_distance;
}

pub fn solve2<P>(filename: P) -> i32 
where P: AsRef<Path>,{
    let data = read_data(filename);
    let mut current_depth = 0;
    let mut current_distance = 0;
    let mut current_aim = 0;
    for movement in data {
        let direction = &movement[0];
        let distance = movement[1].parse::<i32>().unwrap();
        if distance == 0 {
            continue;
        }
        if direction == "forward" {
            current_distance = current_distance + distance;
            current_depth = current_depth + current_aim * distance;
        }
        if direction == "up" {
            current_aim =  current_aim - distance;
        }
        if direction == "down" {
            current_aim = current_aim + distance;
        }
    }
    return current_depth * current_distance;
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

    #[test]
    fn it_should_solve_testdata_for_part_1() {
        assert_eq!(super::solve1("././data/test"),150);
    }

    #[test]    
    fn it_should_solve_testdata_for_part_2() {
        assert_eq!(super::solve2("././data/test"),900);
    }
}
