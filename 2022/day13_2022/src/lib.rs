use std::path::Path;

pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let input = common::read_lines(filename);
    let no_blanks: Vec<&String> = input
        .iter()
        .filter(|line| line != &&"".to_string())
        .collect();
    let mut score = 0;
    let mut i = 0;
    let mut index = 1;
    while i < no_blanks.len() - 1 {
        if compare(no_blanks[i], no_blanks[i + 1]) {
            score += index;
        } else {
        }
        index += 1;
        i += 2;
    }
    score
}

fn compare(a: &String, b: &String) -> bool {
    let a_chars: Vec<String> =
        common::split_lines_no_empty_strings(vec![a.to_string()], "")[0].clone();
    let b_chars: Vec<String> =
        common::split_lines_no_empty_strings(vec![b.to_string()], "")[0].clone();

    let sets: Vec<Vec<String>> = vec![];
    fn print_parts(
        a: &[String],
        mut i: usize,
        mut sets: Vec<Vec<String>>,
    ) -> (usize, Vec<Vec<String>>) {
        let mut set = vec![];
        i += 1;
        while i < a.len() {
            let c = &a[i];
            if c == &'['.to_string() {
                let (increment, sets_1) = print_parts(&a[0..a.len()], i.clone(), sets);
                sets = sets_1;
                i = increment;
            } else {
                if c == &']'.to_string() {
                    sets.push(set.clone());
                    return (i + 1, sets);
                }
                if c != &','.to_string() {
                    let mut number = c.clone();
                    let mut j = 1;
                    while i + j < a.len()
                        && &a[i + j] != &','.to_string()
                        && &a[i + j] != &'['.to_string()
                        && &a[i + j] != &']'.to_string()
                    {
                        number.push_str(&a[i + j]);
                        j += 1;
                    }
                    set.push(number);
                }
            }
            i += 1;
        }
        (a.len(), sets)
    }
    let mut sets_parsed_a = print_parts(a_chars.as_slice(), 0, sets).1;
    let sets_b: Vec<Vec<String>> = vec![];
    let mut sets_parsed_b = print_parts(b_chars.as_slice(), 0, sets_b).1;
    for i in 0..std::cmp::max(sets_parsed_a.len(), sets_parsed_b.len()) {
        if i >= sets_parsed_b.len() {
            println!("invalid, because right ran out");
            return false;
        }
        if i >= sets_parsed_a.len() {
            println!("valid, because left ran out");

            return true;
        }
        for j in 0..std::cmp::max(sets_parsed_a[i].len(), sets_parsed_b[i].len()) {
            if j >= sets_parsed_b[i].len() {
                println!("invalid, because right ran out");

                return false;
            }
            if j >= sets_parsed_a[i].len() {
                println!("valid, because left ran out");

                return true;
            }
            println!("{:?},{:?}", sets_parsed_a[i][j], sets_parsed_b[i][j]);

            let diff: i32 = sets_parsed_b[i][j].parse::<i32>().unwrap()
                - sets_parsed_a[i][j].parse::<i32>().unwrap();
            if diff > 0 {
                println!("valid, left is smaller");

                return true;
            }
            if diff < 0 {
                println!("invalid, right is smaller");
                return false;
            }
        }
    }
    println!("reach end");
    true
    // is_right_order(a_chars[1], b_chars[1])
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
    fn it_should_compare_correctly_part_1() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(compare(&lines[0], &lines[1]), true);
    }

    #[test]
    fn it_should_compare_correctly_part_2() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(compare(&lines[3], &lines[4]), true);
    }

    #[test]
    fn it_should_compare_correctly_part_3() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(compare(&lines[6], &lines[7]), false);
    }

    #[test]
    fn it_should_compare_correctly_part_4() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(compare(&lines[9], &lines[10]), true);
    }

    #[test]
    fn it_should_compare_correctly_part_5() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(compare(&lines[12], &lines[13]), false);
    }

    #[test]
    fn it_should_compare_correctly_part_6() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(compare(&lines[15], &lines[16]), true);
    }

    #[test]
    fn it_should_compare_correctly_part_7() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(compare(&lines[18], &lines[19]), false);
    }

    #[test]
    fn it_should_compare_correctly_part_8() {
        let lines = common::read_lines("./data/sample");
        assert_eq!(compare(&lines[21], &lines[22]), false);
    }

    #[test]
    fn it_should_compare_correctly_part_9() {
        let lines = common::read_lines("./data/sample2");
        assert_eq!(compare(&lines[0], &lines[1]), true);
    }

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 13)
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
