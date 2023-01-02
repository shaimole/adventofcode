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
                    "*" => a*b,
                    "+" => a+b,
                    "-" => a-b,
                    "/" => a/b,
                    _ => unreachable!(),
                };
                known.insert(monkey[0].clone(),monkey_nr);
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
    0
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
        assert_eq!(solve2("./data/sample"), -1)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), -1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), -1)
    }
}
