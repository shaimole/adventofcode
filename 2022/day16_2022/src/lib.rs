use std::collections::HashMap;
use std::path::Path;

struct Cave {
    key: String,
    edges: Vec<String>,
    flow: u32,
}

struct Node {
    key: String,
    water: u32,
    time_remaining: u32,
    current_rate: u32,
    vistited: Vec<String>,
}

pub fn solve<P>(filename: P, start: String) -> u32
where
    P: AsRef<Path>,
{
    let caves: Vec<Cave> = common::read_lines(filename)
        .iter()
        .map(|line| parse_cave(line))
        .collect();
    let (distances, flow_rates) = create_distance_map(caves);
    let mut max_water = 0;
    let mut stack = vec![];
    let root = Node {
        key: "AA".to_string(),
        water: 0,
        time_remaining: 30,
        current_rate: 0,
        vistited: vec![],
    };
    stack.push(root);
    while stack.len() > 0 {
        let current = stack.pop().unwrap();
        max_water = std::cmp::max(max_water, current.water);
        let mut branches = 0;
        for (next, distance) in distances.get(&current.key).unwrap() {
            if current.time_remaining < distance + 1 {
                continue;
            }
            if current.vistited.contains(next) {
                continue;
            }
            let mut next_visisted = current.vistited.clone();
            next_visisted.push(next.clone());
            let next_on_stack = Node {
                key: next.to_string(),
                water: current.water + current.current_rate * (distance + 1),
                time_remaining: current.time_remaining - (distance + 1),
                current_rate: flow_rates.get(next).unwrap() + current.current_rate,
                vistited: next_visisted,
            };
            stack.push(next_on_stack);
            branches += 1;
        }
        if branches == 0 {
            max_water = std::cmp::max(
                max_water,
                current.water + current.current_rate * current.time_remaining,
            );
        }
    }
    max_water
    // let relevant: Vec<&Cave> = caves.iter().filter(|cave| cave.flow != 0).collect();
    // create a hashmap where key = cave key and value = hashmap of distances to all other caves
    // create a hashmap where key = cave flow rate
    // try dfs
    // for each cave the direct edges are distance 1
    // save and check distances directly in hashmap to reduce travel overhead
    // for each cave the distance to any other cave is lowest distance of ther direct edges to that cave +1
    // delete all items where flow rate 0 from hashmap
    // create all different orders of the remaining caves
    // loop all order
    // distance to AA is 1
    // while time is less than 30
    // add to current water level depending on current rate
    // go to next cave and turn on flow rate
    // increase time depending on distance + 1 (open valve)
    // record max flow rate
    // return max flow rate
}

fn create_distance_map(
    caves: Vec<Cave>,
) -> (HashMap<String, HashMap<String, u32>>, HashMap<String, u32>) {
    let mut distances = HashMap::new();
    let mut flow_rates = HashMap::new();
    caves.iter().for_each(|cave| {
        flow_rates.insert(cave.key.clone(), cave.flow);
        let mut distances_for_cave = HashMap::new();
        cave.edges.iter().for_each(|other| {
            distances_for_cave.insert(other.clone(), 1);
        });
        distances.insert(cave.key.clone(), distances_for_cave);
    });
    caves.iter().for_each(|cave| {
        caves.iter().for_each(|other| {
            let distance = get_cave_distance(vec![&cave.key], &other.key, &distances, 0);
            let cave_distances = distances.get_mut(&cave.key).unwrap();
            cave_distances.insert(other.key.clone(), distance);
        })
    });
    caves.iter().for_each(|cave| {
        let flow_rate = flow_rates.get(&cave.key).unwrap();
        if flow_rate == &0 && &cave.key != "AA" {
            distances.remove(&cave.key);
        } else {
            caves.iter().for_each(|other| {
                let flow_rate = flow_rates.get(&other.key).unwrap();
                if flow_rate == &0 {
                    let cave_distances = distances.get_mut(&cave.key).unwrap();
                    cave_distances.remove(&other.key);
                }
            })
        }
    });
    (distances, flow_rates)
}

fn get_cave_distance<'a>(
    starts: Vec<&String>,
    end: &String,
    distances: &'a HashMap<String, HashMap<String, u32>>,
    distance: u32,
) -> u32 {
    if starts.contains(&end) {
        return distance;
    }
    let mut collective_edges = vec![];
    for start in starts {
        let edges = distances.get(start).unwrap();
        for (edge, dist) in edges {
            if dist == &1.try_into().unwrap() {
                collective_edges.push(edge);
            }
        }
    }
    get_cave_distance(collective_edges, end, distances, distance + 1)
}

fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    use itertools::Itertools;
    let caves: Vec<Cave> = common::read_lines(filename)
        .iter()
        .map(|line| parse_cave(line))
        .collect();
    let (distances, flow_rates) = create_distance_map(caves);
    let mut max_water = 0;
    for i in 8..=distances.len() / 2 {
        // educated guess split in 2 xd
        let mut max_water_comb = 0;
        distances.iter().combinations(i).for_each(|allowed| {
            let mut max_water_comb_1 = 0;
            let mut max_water_comb_2 = 0;

            let mut stack = vec![];
            let root = Node {
                key: "AA".to_string(),
                water: 0,
                time_remaining: 26,
                current_rate: 0,
                vistited: vec![],
            };
            stack.push(root);
            while stack.len() > 0 {
                let current = stack.pop().unwrap();
                max_water_comb_1 = std::cmp::max(max_water_comb_1, current.water);
                let mut branches = 0;
                for (next, distance) in distances.get(&current.key).unwrap() {
                    if current.time_remaining < distance + 1 {
                        continue;
                    }
                    if current.vistited.contains(next) {
                        continue;
                    }
                    if allowed.iter().filter(|(k, _)| k == &next).count() == 0 {
                        continue;
                    }
                    let mut next_visisted = current.vistited.clone();
                    next_visisted.push(next.clone());
                    let next_on_stack = Node {
                        key: next.to_string(),
                        water: current.water + current.current_rate * (distance + 1),
                        time_remaining: current.time_remaining - (distance + 1),
                        current_rate: flow_rates.get(next).unwrap() + current.current_rate,
                        vistited: next_visisted,
                    };
                    stack.push(next_on_stack);
                    branches += 1;
                }
                if branches == 0 {
                    max_water_comb_1 = std::cmp::max(
                        max_water_comb_1,
                        current.water + current.current_rate * current.time_remaining,
                    );
                }
            }

            let mut stack = vec![];
            let root = Node {
                key: "AA".to_string(),
                water: 0,
                time_remaining: 26,
                current_rate: 0,
                vistited: vec![],
            };
            stack.push(root);
            while stack.len() > 0 {
                let current = stack.pop().unwrap();
                max_water_comb_2 = std::cmp::max(max_water_comb_2, current.water);
                let mut branches = 0;
                for (next, distance) in distances.get(&current.key).unwrap() {
                    if current.time_remaining < distance + 1 {
                        continue;
                    }
                    if current.vistited.contains(next) {
                        continue;
                    }
                    if allowed.iter().filter(|(k, _)| k == &next).count() > 0 {
                        continue;
                    }
                    let mut next_visisted = current.vistited.clone();
                    next_visisted.push(next.clone());
                    let next_on_stack = Node {
                        key: next.to_string(),
                        water: current.water + current.current_rate * (distance + 1),
                        time_remaining: current.time_remaining - (distance + 1),
                        current_rate: flow_rates.get(next).unwrap() + current.current_rate,
                        vistited: next_visisted,
                    };
                    stack.push(next_on_stack);
                    branches += 1;
                }
                if branches == 0 {
                    max_water_comb_2 = std::cmp::max(
                        max_water_comb_2,
                        current.water + current.current_rate * current.time_remaining,
                    );
                }
            }
            max_water_comb = std::cmp::max(max_water_comb, max_water_comb_1 + max_water_comb_2);
        });
        max_water = std::cmp::max(max_water, max_water_comb);
    }
    max_water
}

fn get_flow_rate(line: &String) -> u32 {
    use regex::Regex;
    let re: Regex = Regex::new(r"-?\d+").unwrap();
    re.find(line).unwrap().as_str().parse().unwrap()
}

fn get_cave_keys(line: &String) -> Vec<String> {
    use regex::Regex;
    let re: Regex = Regex::new(r"[A-Z]{2}").unwrap();
    re.find_iter(line)
        .map(|cave_key| cave_key.as_str().to_string())
        .collect()
}
fn parse_cave(line: &String) -> Cave {
    let double_uppercases = get_cave_keys(line);
    Cave {
        flow: get_flow_rate(line),
        key: double_uppercases.first().unwrap().to_string(),
        edges: double_uppercases
            .iter()
            .skip(1)
            .map(|key| key.to_string())
            .collect(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_parse_flow_rate() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(get_flow_rate(&lines[0]), 0);
        assert_eq!(get_flow_rate(&lines[1]), 13);
    }

    #[test]
    fn it_should_parse_cave_and_edges() {
        let lines = common::read_lines("./data/sample");
        let expect = Cave {
            key: "AA".to_string(),
            edges: vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
            flow: 0,
        };
        let actual = parse_cave(&lines[0]);
        assert_eq!(expect.key, actual.key);
        assert_eq!(expect.edges, actual.edges);
        assert_eq!(expect.flow, actual.flow);
    }

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample", "AA".to_string()), 1651)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 1707)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input", "TM".to_string()), 5716881)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 1707)
    }
}
