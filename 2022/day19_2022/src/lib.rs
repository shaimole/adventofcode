use std::collections::HashSet;
use std::path::Path;

struct Blueprint {
    costs: Vec<Vec<u32>>,
}
struct State {
    income: Vec<u32>,
    amount: Vec<u32>,
    time: i32,
}
pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}

fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}

fn parse<P>(filename: P) -> Vec<Blueprint>
where
    P: AsRef<Path>,
{
    common::read_lines(filename)
        .iter()
        .map(|line| {
            let costs = extract_costs(line);
            Blueprint {
                costs: vec![
                    vec![costs[1], 0, 0, 0],
                    vec![costs[2], 0, 0, 0],
                    vec![costs[3], costs[4], 0, 0],
                    vec![costs[5], costs[6], costs[7], 0],
                ],
            }
        })
        .collect()
}

fn extract_costs(line: &String) -> Vec<u32> {
    use regex::Regex;
    let re: Regex = Regex::new(r"\d").unwrap();
    re.find_iter(line)
        .map(|cost| cost.as_str().parse().unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 33)
    }

    #[test]
    fn it_should_parse_blueprint() {
        let expected = Blueprint {
            costs: vec![
                vec![4, 0, 0, 0],
                vec![2, 0, 0, 0],
                vec![3, 1, 0, 0],
                vec![4, 2, 7, 0],
            ],
        };
        assert_eq!(parse("./data/poc")[0].costs, expected.costs);
    }
    #[test]
    fn it_should_solve_poc() {
        assert_eq!(solve("./data/poc"), 9)
    }
    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 1)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 1)
    }
}
