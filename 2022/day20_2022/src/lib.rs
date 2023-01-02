use std::path::Path;
fn parse<P>(filename: P) -> Vec<(usize, i128)>
where
    P: AsRef<Path>,
{
    common::read_lines(filename)
        .iter()
        .map(|line| line.parse().unwrap())
        .enumerate()
        .collect()
}

pub fn solve<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    let mut code = parse(filename);

    let len = code.len();
    for i in 0..code.len() {
        rotate(&mut code, i);
    }
    for i in 0..code.len() {
        if code[i].1 == 0 {
            println!(
                "{:?}",
                vec![
                    code[(i + 1000) % len].1,
                    code[(i + 2000) % len].1,
                    code[(i + 3000) % len].1
                ]
            );
            return code[(i + 1000) % len].1 + code[(i + 2000) % len].1 + code[(i + 3000) % len].1;
        }
    }
    unreachable!();
    0
}

fn rotate(code: &mut Vec<(usize, i128)>, i: usize) {
    let len = code.len();

    for j in 0..code.len() {
        if code[j].0 == i {
            let to_move = code[j].clone();
            // println!("moving {:?}", to_move.1);
            // println!("moving from {:?}", j);

            let moving = to_move.1 % len as i128;
            // println!("moving distance {:?}", moving);

            let mut new_pos = j as i128 + moving;
            // println!("new pos {:?}", new_pos);

            code.remove(j);
            let offset = to_move.1 / len as i128;
            if offset != 0 {
                // println!("care for self");
            }
            if new_pos <= 0 {
                new_pos += (len - 1) as i128;
            }
            if new_pos == 0 {
                new_pos = len as i128 - 1;
            }
            if new_pos > len as i128 {
                new_pos += 1;
            }
            new_pos += offset;

            new_pos %= len as i128;
            if new_pos <= 0 {
                new_pos += (len - 1) as i128;
            }
            // println!("new pos bounded{:?}", new_pos);

            code.insert(new_pos as usize, to_move);
            break;
        }
    }
}

pub fn solve2<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    let multi = 1;
    let mut code: Vec<(usize, i128)> = parse(filename)
        .iter()
        .map(|(k, v)| (*k, v * 811589153))
        .collect();

    let len = code.len();
    for i in 0..code.len() * 10 {
        let to_rotate = i % len;
        rotate(&mut code, to_rotate);
    }
    for i in 0..code.len() {
        if code[i].1 == 0 {
            println!(
                "{:?}",
                vec![
                    code[(i + 1000) % len].1,
                    code[(i + 2000) % len].1,
                    code[(i + 3000) % len].1
                ]
            );
            return ((code[(i + 1000) % len].1 * multi)
                + (code[(i + 2000) % len].1 * multi)
                + (code[(i + 3000) % len].1) * multi);
        }
    }
    unreachable!();
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 3)
    }

    #[test]
    fn it_should_rotate_correctly() {
        let mut code = parse("./data/sample");
        rotate(&mut code, 0);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![2, 1, -3, 3, -2, 0, 4]);

        rotate(&mut code, 1);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, -3, 2, 3, -2, 0, 4]);

        rotate(&mut code, 2);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, 2, 3, -2, -3, 0, 4]);

        rotate(&mut code, 3);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, 2, -2, -3, 0, 3, 4]);

        rotate(&mut code, 4);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, 2, -3, 0, 3, 4, -2]);

        rotate(&mut code, 5);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, 2, -3, 0, 3, 4, -2]);

        rotate(&mut code, 6);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn it_should_rotate_correctly_edge() {
        let mut code = parse("./data/edge");
        rotate(&mut code, 0);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, -1, 1]);

        rotate(&mut code, 1);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![-1, 1, 1]);

        rotate(&mut code, 2);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, -1, 1]);
    }

    #[test]
    fn it_should_rotate_correctly_edge2() {
        let mut code = parse("./data/edge2");
        rotate(&mut code, 0);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, -1, 1]);

        rotate(&mut code, 1);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![-1, 1, 1]);

        rotate(&mut code, 2);
        let numbers: Vec<i128> = code.iter().map(|(_, v)| *v).collect();
        assert_eq!(numbers, vec![1, -1, 1]);
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"), 1623178306)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 1789)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 314820)
    }
}
