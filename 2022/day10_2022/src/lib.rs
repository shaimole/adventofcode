use std::path::Path;

fn register_values<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let mut register_value: Vec<i32> = vec![1];
    let instructions = common::split_lines(common::read_lines(filename), " ");
    instructions
        .iter()
        .for_each(|instruction| match instruction[0].as_str() {
            "addx" => {
                register_value.push(*register_value.last().unwrap());
                register_value
                    .push(*register_value.last().unwrap() + instruction[1].parse::<i32>().unwrap())
            }
            "noop" => register_value.push(*register_value.last().unwrap()),
            _ => unreachable!(),
        });
    register_value
}
pub fn solve<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let register_value: Vec<i32> = register_values(filename);
    register_value
        .iter()
        .enumerate()
        .filter(|(i, _)| i > &18 && (i - 19) % 40 == 0)
        .map(|(i, value)| value * (i + 1) as i32)
        .sum()
}

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let register_value: Vec<i32> = register_values(filename);
    register_value.iter().enumerate().for_each(|(i, value)| {
        if i % 40 == 0 && i > 39 {
            println!("");
        }
        let distance_to_i = value - (i as i32) % 40;
        if distance_to_i <= 1 && distance_to_i >= -1 {
            print!("#");
        } else {
            print!(".");
        }
    });

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 13140)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 1)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 15020)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 1)
    }
}
