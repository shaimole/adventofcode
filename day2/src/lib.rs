use common;
use std::path::Path;


pub fn solve1<P>(filename: P) -> i32 
where P: AsRef<Path>,{
    let data = parse_data(filename);
    let mut submarine = create_submarine();
    for movement in data {
        submarine = submarine.travel(&movement[0], movement[1].parse::<i32>().unwrap());
    }
    return submarine.depth * submarine.distance;
}

fn parse_data<P>(filename: P) -> Vec<Vec<String>> 
where P: AsRef<Path>, {
    let lines = common::read_lines(filename);
    return common::split_lines(lines," ");
}

fn create_submarine() -> Submarine {
    return Submarine {
        depth : 0,
        distance: 0,
        aim : 0
    }
}

pub fn solve2<P>(filename: P) -> i32 
where P: AsRef<Path>,{
    let data = parse_data(filename);
    let mut submarine = create_submarine();
    for movement in data {
        submarine = submarine.aim_and_travel(&movement[0], movement[1].parse::<i32>().unwrap());
    }
    return submarine.depth * submarine.distance;
}


struct Submarine {
    depth: i32,
    distance: i32,
    aim: i32
}

impl Submarine {

    fn aim_and_travel(mut self, direction: &str, distance: i32) -> Submarine {
        if distance == 0 {
            return self;
        } else if direction == "forward" {
            self.distance += distance;
            self.depth += self.aim * distance;
         }else if direction == "up" {
            self.aim -= distance;
        } else if direction == "down" {
            self.aim += distance;
        }
        return self;
    }

    fn travel(mut self, direction: &str, distance: i32) -> Submarine {
        if distance == 0 {
            return self
        }else if direction == "forward" {
            self.distance += distance;
        }else if direction == "up" {
            self.depth -= distance;
        }else if direction == "down" {
            self.depth += distance;
        }
        return self;
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_testfile() {
        let result = super::parse_data("././data/test");
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
