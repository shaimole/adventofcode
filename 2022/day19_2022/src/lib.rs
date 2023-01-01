use std::path::Path;
#[derive(Clone, Debug)]
struct Blueprint {
    costs: Vec<Vec<i32>>,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Ressources {
    income: Vec<i32>,
    amount: Vec<i32>,
    time: i32,
}

impl Ressources {
    fn possible_robots(&self, blueprint: &Blueprint, score: &i32) -> Vec<Ressources> {
        let mut robots: Vec<Ressources> = vec![];
        for (index, cost) in blueprint.costs.iter().enumerate().rev() {
            let would_be_excess_res = index != 3
                && self.income[index] >= blueprint.costs[0][index]
                && self.income[index] >= blueprint.costs[1][index]
                && self.income[index] >= blueprint.costs[2][index]
                && self.income[index] >= blueprint.costs[3][index];
            // println!("{index} excess {:?}", would_be_excess_res);
            if would_be_excess_res {
                continue;
            }
            let mut time_to_build = 1;
            let res_needed_to_build = Ressources::diff(&self.amount, cost);
            let can_build_now = res_needed_to_build.iter().filter(|v| v >= &&0).count() == 4;
            // println!("{index} can build now {:?}", can_build_now);

            if !can_build_now {
                let can_build_at_some_point = Ressources::can_build_with_time(
                    &self.income,
                    &res_needed_to_build,
                    &(self.time - 1),
                );
                // println!("{index} can build at some point {:?}", can_build_now);

                if !can_build_at_some_point {
                    continue;
                }
                let mut time_to_build_min = self.time;
                while Ressources::can_build_with_time(
                    &self.income,
                    &res_needed_to_build,
                    &time_to_build_min,
                ) {
                    time_to_build_min -= 1;
                }
                // println!("building in  {time_to_build_min} min ");

                time_to_build = time_to_build_min;
                time_to_build += 2; // one for building round one for moving to next min
            }
            let mut n_inc = self.income.clone();
            n_inc[index] += 1;
            let new_amt = self
                .amount
                .iter()
                .enumerate()
                .map(|(i, v)| v + self.income[i] * time_to_build - cost[i])
                .collect();
            let new_state = Ressources {
                income: n_inc,
                amount: new_amt,
                time: self.time - time_to_build,
            };
            robots.push(new_state);
            if index == 3 && can_build_now {
                return robots;
            }
        }
        robots
    }
    fn diff(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        let mut sum = vec![];
        for (aval, bval) in a.iter().zip(b) {
            sum.push(aval - bval);
        }
        sum
    }

    fn can_build_with_time(income: &Vec<i32>, cost: &Vec<i32>, time: &i32) -> bool {
        cost.iter()
            .enumerate()
            .map(|(i, v)| v + income[i] * time)
            .filter(|v| v >= &0)
            .count()
            == 4
    }
}
pub fn solve<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let blueprints = parse(filename);
    // println!("{:?}", blueprints);
    let mut scores: Vec<i32> = vec![];
    for blueprint in blueprints {
        let mut score = 0;

        let mut stack: Vec<Ressources> = vec![Ressources {
            amount: vec![0, 0, 0, 0],
            income: vec![1, 0, 0, 0],
            time: 24,
        }];
        while !stack.is_empty() {
            let state = stack.pop().unwrap();
            // println!("{:?}", state.time);
            if state.time <= 0 {
                // println!("{:?}", state);
                score = std::cmp::max(score, state.amount[3]);
                continue;
            }
            for new_state in state.possible_robots(&blueprint, &score) {
                stack.push(new_state);
            }
        }
        scores.push(score);
    }
    println!("{:?}", scores);
    scores
        .iter()
        .enumerate()
        .map(|(k, v)| (k + 1) as i32 * v)
        .sum()
}

fn solve2<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let blueprints = parse(filename);
    // println!("{:?}", blueprints);
    let mut scores: Vec<i32> = vec![];
    for blueprint in blueprints.iter().take(3) {
        let mut score = 0;

        let mut stack: Vec<Ressources> = vec![Ressources {
            amount: vec![0, 0, 0, 0],
            income: vec![1, 0, 0, 0],
            time: 32,
        }];
        while !stack.is_empty() {
            let state = stack.pop().unwrap();
            // println!("{:?}", state.time);
            if state.time <= 0 {
                // println!("{:?}", state);
                score = std::cmp::max(score, state.amount[3]);
                continue;
            }
            for new_state in state.possible_robots(&blueprint, &score) {
                stack.push(new_state);
            }
        }
        scores.push(score);
    }
    println!("{:?}", scores);
    scores[0] * scores[1] * scores[2]
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
                    vec![costs[5], 0, costs[6], 0],
                ],
            }
        })
        .collect()
}

fn extract_costs(line: &String) -> Vec<i32> {
    use regex::Regex;
    let re: Regex = Regex::new(r"[0-9]+").unwrap();
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
            time: 24,
        };

        assert_eq!(
            res.possible_robots(&bp, &0),
            vec![
                Ressources {
                    amount: vec![1, 0, 0, 0],
                    income: vec![2, 0, 0, 0],
                    time: 19,
                },
                Ressources {
                    amount: vec![1, 0, 0, 0],
                    income: vec![1, 1, 0, 0],
                    time: 21,
                }
            ]
        );
        let res2 = Ressources {
            amount: vec![10, 10, 10, 0],
            income: vec![1, 2, 2, 0],
            time: 20,
        };
        assert_eq!(
            res2.possible_robots(&bp, &0),
            vec![
                Ressources {
                    amount: vec![7, 12, 12, 0],
                    income: vec![2, 2, 2, 0],
                    time: 19,
                },
                Ressources {
                    amount: vec![8, 11, 12, 0],
                    income: vec![1, 2, 3, 0],
                    time: 19,
                },
                Ressources {
                    amount: vec![7, 10, 5, 0],
                    income: vec![1, 2, 2, 1],
                    time: 19,
                }
            ]
        );
    }

    #[test]
    fn it_should_solve_poc() {
        assert_eq!(solve("./data/poc"), 9)
    }

    #[test]
    fn it_should_solve_poc2() {
        assert_eq!(solve2("./data/poc"), 56)
    }
    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), -1)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 1192)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 1)
    }
}
