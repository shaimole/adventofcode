use std::collections::HashSet;
use std::path::Path;

fn move_tail(mut tail_pos: (i32, i32), head_pos: (i32, i32)) -> (i32, i32) {
    let diff = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);
    if diff.1.abs() <= 1 && diff.0.abs() <= 1 {
        return tail_pos;
    }

    tail_pos.0 += diff.0.clamp(-1, 1);
    tail_pos.1 += diff.1.clamp(-1, 1);
    tail_pos
}

pub fn solve<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    let instructions = common::split_lines(common::read_lines(filename), " ");
    let mut tail_pos = (0, 0);
    let mut head_pos = (0, 0);

    tail_visited.insert(tail_pos);
    instructions.iter().for_each(|line| {
        let direction: &str = &line[0];
        let steps: u32 = line[1].parse().unwrap();
        for _ in 0..steps {
            match direction {
                "R" => head_pos.1 += 1,
                "L" => head_pos.1 -= 1,
                "U" => head_pos.0 -= 1,
                "D" => head_pos.0 += 1,
                _ => unreachable!(),
            }
            tail_pos = move_tail(tail_pos, head_pos);
            tail_visited.insert(tail_pos);
        }
    });
    tail_visited.iter().len()
}

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 13)
    }
    #[test]
    fn it_should_solve_sample_extended() {
        assert_eq!(solve("./data/sample2"), 24)
    }
    #[test]
    fn it_should_move_tail_correctly() {
        assert_eq!(move_tail((0, 0), (1, 0)), (0, 0));
        assert_eq!(move_tail((0, 0), (1, 1)), (0, 0));
        assert_eq!(move_tail((0, 0), (0, 1)), (0, 0));
        assert_eq!(move_tail((0, 0), (-1, 1)), (0, 0));
        assert_eq!(move_tail((0, 0), (-1, 0)), (0, 0));
        assert_eq!(move_tail((0, 0), (-1, -1)), (0, 0));
        assert_eq!(move_tail((0, 0), (0, -1)), (0, 0));
        assert_eq!(move_tail((0, 0), (1, 1)), (0, 0));
        assert_eq!(move_tail((0, 0), (2, 0)), (1, 0));
        assert_eq!(move_tail((0, 0), (2, 1)), (1, 1));
        assert_eq!(move_tail((0, 0), (2, 2)), (1, 1));
        assert_eq!(move_tail((0, 0), (1, 2)), (1, 1));
        assert_eq!(move_tail((0, 0), (0, 2)), (0, 1));
        assert_eq!(move_tail((0, 0), (-1, 2)), (-1, 1));
        assert_eq!(move_tail((0, 0), (-2, 2)), (-1, 1));
        assert_eq!(move_tail((0, 0), (-2, 1)), (-1, 1));
        assert_eq!(move_tail((0, 0), (-2, 0)), (-1, 0));
        assert_eq!(move_tail((0, 0), (-2, 1)), (-1, 1));
        assert_eq!(move_tail((0, 0), (-2, 2)), (-1, 1));
        assert_eq!(move_tail((0, 0), (-1, 2)), (-1, 1));
        assert_eq!(move_tail((0, 0), (0, -2)), (0, -1));
        assert_eq!(move_tail((0, 0), (1, -2)), (1, -1));
        assert_eq!(move_tail((0, 0), (2, -2)), (1, -1));
        assert_eq!(move_tail((0, 0), (2, -1)), (1, -1));
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"), 8)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 6354)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 314820)
    }
}
