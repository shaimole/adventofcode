use std::path::Path;

fn parse<P>(filename: P) -> Vec<(usize, i32)>
where
    P: AsRef<Path>,
{
    common::read_lines(filename)
        .iter()
        .map(|line| line.parse().unwrap())
        .enumerate()
        .collect()
}

pub fn solve<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let mut code = parse(filename);
    let len = code.len();
    for i in 0..code.len() {
        for j in 0..code.len() {
            if code[j].0 == i {

    println!("{:?}", i);
                let to_move = code[j].clone();
                let mut new_pos = j as i32 + to_move.1% len as i32;
    println!("{:?}", new_pos);
    println!("{:?}",j);
                if new_pos < 0 {
                    new_pos += len as i32;
                }
                new_pos %= len as i32;
                code.remove(j);

                code.insert(new_pos as usize, to_move);
    println!("{:?}", new_pos);
    println!("{:?}", code);
    break;
            }
        }
    }
    println!("{:?}", code[1004%len]);
    for i in 0..code.len() {
        if code[i].1 == 0 {
            println!("{:?}",vec![ code[(i+1000)%len].1
            , code[(i+2000)%len].1
            , code[(i+3000)%len].1]);
            return code[(i+1000)%len].1
            + code[(i+2000)%len].1
            + code[(i+3000)%len].1
        }
    }
    unreachable!();
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
        assert_eq!(solve("./data/sample"), 3)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"), 8)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 1789)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 314820)
    }
}
