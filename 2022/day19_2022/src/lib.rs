use std::collections::HashSet;
use std::path::Path;
#[derive(Clone, Debug)]
struct Blueprint {
    costs: Vec<Vec<i32>>,
}
#[derive(Clone, Debug)]
struct Ressources {
    income: Vec<i32>,
    amount: Vec<i32>,
    time: i32,
}

impl Ressources {
    fn possible_robots(&self, blueprint: &Blueprint) -> Vec<(usize, i32)> {
        let mut robots: Vec<Ressources> = vec![];
        for cost in blueprint.costs.iter().enumerate() {
            let would_be_excess_res = false;
            if would_be_eccess_res {
                continue;
            }
            let mut time_to_build = 1;
            let mut new_res = *self.clone(); 
            let res_needed_to_build = Ressources::diff(self.amount().iter().enumerate().collect(),cost);
            let can_build_now = Ressources::diff(self.amount().iter().enumerate().collect(),cost).filter(Ressources::all_positive).count() == 4;
            if !can_build_now {
        
                let can_build_at_some_point = Ressources::diff(self.amount().iter().enumerate().collect(),cost).filter(Ressources::all_positive).count() == 4;
                if !can_build_at_some_point {
                    continue;
                }
                time_to_build = 2;
            }
            let 
        }
        let res_available: Vec<i32> = self
            .amount
            .iter()
            .enumerate()
            .map(|(i, amount_available)| amount_available + self.income[i] * self.time)
            .collect();
        blueprint
            .costs
            .iter()
            .map(|costs| Ressources::diff(&res_available, costs))
            .enumerate()
            .filter(|(i, v)| Ressources::all_positive(v))
            .map(|(i, v)| (i, Ressources::execess_time(&v, &self.income)))
            .collect()
    }
    fn diff(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        let mut sum = vec![];
        for (aval, bval) in a.iter().zip(b) {
            sum.push(aval - bval);
        }
        sum
    }

    fn execess_time(excess_res: &Vec<i32>, income: &Vec<i32>) -> i32 {
        excess_res
            .iter()
            .enumerate()
            .filter(|(i, _)| income[*i] > 0)
            .map(|(i, v)| return v / income[i])
            .max()
            .unwrap()
    }
    fn all_positive(res_cost: &Vec<i32>) -> bool {
        res_cost.iter().filter(|v| v >= &&0).count() == 4
    }
}
pub fn solve<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let blueprints = parse(filename);
    let mut scores: Vec<i32> = vec![];
    let mut stack: Vec<Ressources> = vec![Ressources {
        amount: vec![0, 0, 0, 0],
        income: vec![1, 0, 0, 0],
        time: 26,
    }];
    let mut score = 0;
    let blueprint = blueprints[0].clone();
    while !stack.is_empty() {
        let state = stack.pop().unwrap();
            println!("{:?}",state.time);
        if state.time <= 15 {
            println!("{:?}",state);
            score = std::cmp::max(score, state.amount[3]);
            continue;
        }
        for new_state in state.possible_robots(&blueprint) {
            let mut new_income = state.income.clone();
            new_income[new_state.0] += 1;
            let time_passed = state.time - new_state.1+1;
            let mut new_amount: Vec<i32> = vec![];
            for (i,inc) in state.income.iter().enumerate() {
                let new_r_v = state.amount[i] + inc * time_passed - blueprint.costs[new_state.0][i];
                new_amount.push(new_r_v);
            }
            println!("new res{:?}",new_amount);
            println!("new income {:?}",new_income);
            println!("time spend{:?}",time_passed);
            stack.push(Ressources {
                income: new_income,
                amount: new_amount,
                time: state.time- time_passed
            });
        }
    }
    score
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

        assert_eq!(res.possible_robots(&bp), vec![(0, 22), (1, 24)]);
        let res2 = Ressources {
            amount: vec![10, 10, 10, 0],
            income: vec![1, 2, 2, 0],
            time: 20,
        };
        assert_eq!(res2.possible_robots(&bp), vec![(0, 19), (1, 19),(2, 19),(3, 19)]);
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
