use std::path::Path;

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let lines = common::read_lines(filename);
    let mut no_blanks: Vec<String> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .cloned()
        .collect();
    no_blanks.extend(vec!["[[2]]".to_string(), "[[6]]".to_string()]);
    no_blanks.sort_by(|a, b| {
        if compare(
            &serde_json::from_str(a).unwrap(),
            &serde_json::from_str(b).unwrap(),
        ) > 0
        {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Greater;
    });
    let mut product = 1;
    for i in 0..no_blanks.len() {
        if no_blanks[i] == "[[2]]".to_string() || no_blanks[i] == "[[6]]".to_string() {
            product *= i + 1;
        }
    }
    product as u32
}

fn solve<P>(filename: P) -> u64
where
    P: AsRef<Path>,
{
    let lines = common::read_lines(filename);
    let mut sum = 0;
    let mut touple_index = 1;
    let mut i = 1;
    while i < lines.len() - 1 {
        if compare(
            &serde_json::from_str(&lines[i - 1]).unwrap(),
            &serde_json::from_str(&lines[i]).unwrap(),
        ) > 0
        {
            sum += touple_index;
        }
        i += 3;
        touple_index += 1;
    }
    sum as u64
}

fn compare(a: &serde_json::Value, b: &serde_json::Value) -> i8 {
    if !a.is_array() && !b.is_array() {
        let diff: i64 = b.as_i64().unwrap() - a.as_i64().unwrap();
        if diff > 0 {
            return 1;
        }
        if diff < 0 {
            return -1;
        }
        return 0;
    }

    let (list_a, list_b) = match (a.is_array(), b.is_array()) {
        (true, false) => (a.as_array().unwrap().clone(), vec![b.clone()]),
        (false, true) => (vec![a.clone()], b.as_array().unwrap().clone()),
        (true, true) => (a.as_array().unwrap().clone(), b.as_array().unwrap().clone()),
        _ => unreachable!(),
    };
    for i in 0..std::cmp::max(list_a.len(), list_b.len()) {
        if i >= list_b.len() {
            return -1;
        }
        if i >= list_a.len() {
            return 1;
        }
        let res = compare(&list_a[i], &list_b[i]);
        if res != 0 {
            return res;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_compare_correctly_part_1() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(
            compare(
                &serde_json::from_str(&lines[0]).unwrap(),
                &serde_json::from_str(&lines[1]).unwrap()
            ) > 0,
            true
        );
    }

    #[test]
    fn it_should_compare_correctly_part_2() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(
            compare(
                &serde_json::from_str(&lines[3]).unwrap(),
                &serde_json::from_str(&lines[4]).unwrap()
            ) > 0,
            true
        );
    }

    #[test]
    fn it_should_compare_correctly_part_3() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(
            compare(
                &serde_json::from_str(&lines[6]).unwrap(),
                &serde_json::from_str(&lines[7]).unwrap()
            ) > 0,
            false
        );
    }

    #[test]
    fn it_should_compare_correctly_part_4() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(
            compare(
                &serde_json::from_str(&lines[9]).unwrap(),
                &serde_json::from_str(&lines[10]).unwrap()
            ) > 0,
            true
        );
    }

    #[test]
    fn it_should_compare_correctly_part_5() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(
            compare(
                &serde_json::from_str(&lines[12]).unwrap(),
                &serde_json::from_str(&lines[13]).unwrap()
            ) > 0,
            false
        );
    }

    #[test]
    fn it_should_compare_correctly_part_6() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(
            compare(
                &serde_json::from_str(&lines[15]).unwrap(),
                &serde_json::from_str(&lines[16]).unwrap()
            ) > 0,
            true
        );
    }

    #[test]
    fn it_should_compare_correctly_part_7() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(
            compare(
                &serde_json::from_str(&lines[18]).unwrap(),
                &serde_json::from_str(&lines[19]).unwrap()
            ) > 0,
            false
        );
    }

    #[test]
    fn it_should_compare_correctly_part_8() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(
            compare(
                &serde_json::from_str(&lines[21]).unwrap(),
                &serde_json::from_str(&lines[22]).unwrap()
            ) > 0,
            false
        );
    }
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 13)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 140)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 5675)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 20383)
    }
}
