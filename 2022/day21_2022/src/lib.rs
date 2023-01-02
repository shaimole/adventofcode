use std::collections::HashMap;
use std::path::Path;
fn parse<P>(filename: P) -> (HashMap<String, i128>, Vec<Vec<String>>)
where
    P: AsRef<Path>,
{
    let mut known: HashMap<String, i128> = HashMap::new();
    let mut unknown: Vec<Vec<String>> = vec![];
    common::read_lines(filename).iter().for_each(|line| {
        let parsed = extract(line);
        let is_known = parsed.len() == 2;
        if is_known {
            known.insert(parsed[0].clone(), parsed[1].parse().unwrap());
        } else {
            unknown.push(parsed);
        }
    });
    (known, unknown)
}

fn extract(line: &String) -> Vec<String> {
    let is_unknown = line.chars().count() == 17;
    if is_unknown {
        return vec![
            line.chars().take(4).collect(),
            line.chars().skip(6).take(4).collect(),
            line.chars().skip(13).take(4).collect(),
            line.chars().skip(11).take(1).collect(),
        ];
    }
    vec![
        line.chars().take(4).collect(),
        line.chars().skip(6).collect(),
    ]
}

pub fn solve<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    let (mut known, mut unknown) = parse(filename);
    while !known.contains_key(&"root".to_string()) {
        for i in 0..unknown.len() {
            let monkey = unknown[i].clone();
            if known.contains_key(&monkey[1]) && known.contains_key(&monkey[2]) {
                let a: i128 = *known.get(&monkey[1]).unwrap();
                let b: i128 = *known.get(&monkey[2]).unwrap();
                let monkey_nr = match monkey[3].as_str() {
                    "*" => a * b,
                    "+" => a + b,
                    "-" => a - b,
                    "/" => a / b,
                    _ => unreachable!(),
                };
                known.insert(monkey[0].clone(), monkey_nr);
                unknown.remove(i);
                break;
            }
        }
    }

    *known.get(&"root".to_string()).unwrap()
}

pub fn solve2<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    let (mut known, mut unknown) = parse(filename);
    known.remove(&("humn".to_string()));
    let mut prev_ukn = 0;
    while prev_ukn != unknown.len() {
        prev_ukn = unknown.len();
        for i in 0..unknown.len() {
            let monkey = unknown[i].clone();

            if known.contains_key(&monkey[1]) && known.contains_key(&monkey[2]) {
                let a: i128 = *known.get(&monkey[1]).unwrap();
                let b: i128 = *known.get(&monkey[2]).unwrap();
                let monkey_nr = match monkey[3].as_str() {
                    "*" => a * b,
                    "+" => a + b,
                    "-" => a - b,
                    "/" => a / b,
                    _ => unreachable!(),
                };
                known.insert(monkey[0].clone(), monkey_nr);
                unknown.remove(i);
                break;
            }
        }
    }

    println!("{:?}", known);
    let unknown_map: HashMap<String, (String, String, String)> = unknown
        .iter()
        .map(|v| (v[0].clone(), (v[1].clone(), v[2].clone(), v[3].clone())))
        .collect();
    let root = unknown_map.get(&("root".to_string())).unwrap();

    if known.contains_key(&root.1) {
        known.insert(root.0.clone(), *known.get(&root.1).unwrap());
    } else {
        known.insert(root.1.clone(), *known.get(&root.0).unwrap());
    }
    println!("{:?}", known);
    while !known.contains_key(&"humn".to_string()) {
        for (key, unknown) in unknown_map.iter() {
            if !known.contains_key(key) {
                continue;
            }

            if known.contains_key(&unknown.0) {
                let a: i128 = *known.get(key).unwrap();
                let b: i128 = *known.get(&unknown.0).unwrap();
                let monkey_nr = match unknown.2.as_str() {
                    "*" => a / b,
                    "+" => a - b,
                    "-" => a + b,
                    "/" => a * b,
                    _ => unreachable!(),
                };
                known.insert(unknown.1.clone(), monkey_nr);
            }else 
            if known.contains_key(&unknown.1) {
                let a: i128 = *known.get(key).unwrap();
                let b: i128 = *known.get(&unknown.1).unwrap();
                
                let monkey_nr = match unknown.2.as_str() {
                    "*" => a / b,
                    "+" => a - b,
                    "-" => a + b,
                    "/" => a * b,
                    _ => unreachable!(),
                };
                known.insert(unknown.0.clone(), monkey_nr);
            println!("{:?}", (a,b,monkey_nr));
            }
        }
    }
    *known.get(&"humn".to_string()).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 152)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"), 301)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 22382838633806)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), -1)
    }
}
