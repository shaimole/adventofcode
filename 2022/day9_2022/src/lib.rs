use std::collections::HashSet;
use std::path::Path;

fn move_tail(mut tail_pos: (i32, i32), head_pos: (i32, i32)) -> (i32,i32) {
    let total_distance = ((tail_pos.0.abs() - head_pos.0.abs()).abs() + (tail_pos.1.abs() - head_pos.1.abs()).abs()).abs();
    let mut distance_threshhold = 1;
    if (total_distance == 3) {
        distance_threshhold = 0;
    }
    println!("Tail: {:?} Head: {:?}",tail_pos, head_pos);
        println!("distance: {:?} threshold: {:?}",total_distance, distance_threshhold);

    if tail_pos.0 - head_pos.0 > distance_threshhold {
        println!("Move x right");
            tail_pos.0 = tail_pos.0 - 1;
    }
    if tail_pos.0 - head_pos.0 < -distance_threshhold {
                println!("Move x left");
            tail_pos.0 += 1;
    }
    if tail_pos.1 - head_pos.1 > distance_threshhold {
             println!("Move y down");
            tail_pos.1 = tail_pos.1 - 1;
    }

    if tail_pos.1 - head_pos.1 < -distance_threshhold{
         println!("Move y up");
            tail_pos.1 += 1;
    }
    tail_pos
}

pub fn solve<P>(filename : P) -> usize 
where P: AsRef<Path>, {
    let mut tail_visited: HashSet<(i32,i32)> = HashSet::new();
    let instructions = common::split_lines(common::read_lines(filename)," ");
    let mut tail_pos = (0,0);
    let mut head_pos = (0,0);

    

    instructions.iter().for_each(|line| {
        let direction:&str = &line[0];
        let steps: u32 = line[1].parse().unwrap();
        for _ in 0..steps {
            println!("Tail: {:?} Head: {:?}",tail_pos, head_pos);
            tail_visited.insert(tail_pos);
            match direction {
                "R" => {
                    head_pos.1 += 1;
                    tail_pos = move_tail(tail_pos, head_pos);

                },
                "L" => {
                    head_pos.1 -= 1;
                    tail_pos = move_tail(tail_pos, head_pos);
                },
                "U" => {
                    head_pos.0 -= 1;
                    tail_pos = move_tail(tail_pos, head_pos)

                },
                "D" => {
                    head_pos.0 += 1;
                    tail_pos = move_tail(tail_pos, head_pos)
                },
                _ => unreachable!()
            }
        }

    });
    println!("{:?}",tail_visited);

    tail_visited.iter().len()
}


pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"),13)
    }
    #[test]
    fn it_should_move_tail_correctly() {
        assert_eq!(move_tail((0,0),(0,2)),(0,1));
        assert_eq!(move_tail((0,-1),(0,-3)),(0,-2));
        assert_eq!(move_tail((0,0),(0,-2)),(0,-1));
        assert_eq!(move_tail((0,0),(-2,-2)),(-1,-1));
        assert_eq!(move_tail((0,0),(2,-2)),(1,-1));
        assert_eq!(move_tail((0,0),(2,0)),(1,0));
        assert_eq!(move_tail((0,0),(-2,0)),(-1,0));
        assert_eq!(move_tail((0,3),(-2,4)),(-1,4));
        assert_eq!(move_tail((-1,4),(-3,4)),(-2,4));
        assert_eq!(move_tail((-3,4),(-4,2)),(-4,3));
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"),8)
    }
    #[test]
    fn it_should_solve_part_1() {
         assert_eq!(solve("./data/input"),1789)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"),314820)
    }

}
