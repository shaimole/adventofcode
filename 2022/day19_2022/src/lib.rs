use std::collections::HashSet;
use std::path::Path;

struct Blueprint {
    costs: Vec<Vec<i32>>,
}
struct Ressources {
    income: Vec<i32>,
    amount: Vec<i32>,
    time: i32,
}

impl Ressources {
    fn possible_robots(&self, blueprint: &Blueprint) -> Vec<usize> {
        let res_available: Vec<i32> = self
            .amount
            .iter()
            .enumerate()
            .map(|(i, amt)| amt + self.income[i] * self.time)
            .collect();
        blueprint
            .costs
            .iter()
            .map(|costs| Ressources::diff(&res_available, costs))
            .enumerate()
            .filter(|(i, v)| Ressources::all_positive(v))
            .map(|(i, v)| i)
            .collect()
    }
    fn diff(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        let mut sum = vec![];
        for (aval, bval) in a.iter().zip(b) {
            sum.push(aval - bval);
        }
        sum
    }

    fn all_positive(res_cost: &Vec<i32>) -> bool {
        res_cost.iter().filter(|v| v >= &&0).count() == 4
    }
}
pub fn solve<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    0
}

fn solve2<P>(filename: P) -> i32
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

fn extract_costs(line: &String) -> Vec<i32> {
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
    fn it_should_determine_possible_next_robots_with_waiting() {
        let bp = Blueprint {
            costs: vec![
                vec![4, 0, 0, 0],
                vec![2, 0, 0, 0],
                vec![3, 1, 0, 0],
                vec![4, 2, 7, 0],
            ],
        };
        let res = Ressources {
            amount: vec![0, 0, 0, 0],
            income: vec![1, 0, 0, 0],
            time: 26,
        };

        assert_eq!(res.possible_robots(&bp), vec![0, 1]);
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
