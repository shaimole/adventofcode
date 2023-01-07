use std::collections::HashMap;
use std::path::Path;

fn print(
    map: &HashMap<(usize, usize), char>,
    max: &(usize, usize),
    pos: &(usize, usize),
    direction: &usize,
) {
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            if pos == &(x, y) {
                match direction {
                    0 => print!(">"),
                    1 => print!("v"),
                    2 => print!("<"),
                    _ => print!("^"),
                }
                continue;
            }
            if !map.contains_key(&(x, y)) {
                print!(" ");
                continue;
            }
            print!("{}", map.get(&(x, y)).unwrap());
        }
        println!("");
    }
}

fn printcube(
    map: &HashMap<((usize, usize), usize), ((usize, usize), usize)>,
    max: &(usize, usize),
) {
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            if map.contains_key(&((x, y), 1)) {
                assert_eq!(map.get(&((x, y), 1)).unwrap().1, 3);
                print!("v");
            } else if map.contains_key(&((x, y), 2)) {
                print!("<");
            } else if map.contains_key(&((x, y), 0)) {
                print!(">");
            } else if map.contains_key(&((x, y), 3)) {
                print!("^");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

pub fn parse<P>(
    filename: P,
) -> (
    HashMap<(usize, usize), char>,
    (usize, usize),
    Vec<(usize, usize)>,
)
where
    P: AsRef<Path>,
{
    let mut max_x = 0;
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let lines = common::read_lines(filename);
    for y in 0..lines.len() {
        if lines[y].as_str() == "" {
            break;
        }
        for (x, c) in lines[y].chars().enumerate() {
            if c == ' ' {
                continue;
            }
            map.insert((x, y), c);
            max_x = std::cmp::max(x, max_x);
        }
    }
    let movement = parse_movement(&lines[lines.len() - 1]);
    (map, (max_x, lines.len() - 2), movement)
}

fn parse_movement(line: &String) -> Vec<(usize, usize)> {
    let parse = "R".to_owned() + &line.clone();
    use regex::Regex;
    let re: Regex = Regex::new(r"[A-Z][0-9]*").unwrap();
    re.find_iter(&parse)
        .map(|found| {
            let as_str = found.as_str();
            let direction: usize = match as_str.chars().nth(0).unwrap() {
                'R' => 1,
                'L' => 3,
                _ => unreachable!(),
            };

            let amount: usize = as_str[1..].parse().unwrap();
            return (direction, amount);
        })
        .collect()
}
pub fn solve<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let (map, max, movement) = parse(filename);
    let mut direction = 0;
    let directions: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut position = find_outer_edge(&map, &max, &0, &0, &direction);
    direction = 3;
    for (turn, steps) in movement.iter() {
        direction = (direction + turn) % directions.len();
        let velocity = directions[direction];
        for i in 0..*steps {
            //println!("velo {:?}",velocity);
            //println!("ditection  {:?}", direction);
            //println!("pos {:?}",position);
            //println!("");

            let mut target = (
                (position.0 as i64 + velocity.0) as usize,
                (position.1 as i64 + velocity.1) as usize,
            );
            if !map.contains_key(&(target)) {
                target = find_outer_edge(&map, &max, &position.0, &position.1, &direction);
            }
            if map.get(&target).unwrap() == &'#' {
                break;
            }
            position = target;
        }
    }
    print(&map, &max, &position, &direction);

    1000 * (position.1 + 1) + 4 * (1 + position.0) + direction
}

fn find_outer_edge(
    map: &HashMap<(usize, usize), char>,
    max: &(usize, usize),
    x: &usize,
    y: &usize,
    direction: &usize,
) -> (usize, usize) {
    println!("{:?}", direction);
    if direction == &0 {
        for i in 0..=max.0 {
            if map.contains_key(&(i, *y)) {
                return (i, *y);
            }
        }
    }
    if direction == &1 {
        for i in 0..=max.1 {
            if map.contains_key(&(*x, i)) {
                return (*x, i);
            }
        }
    }
    if direction == &2 {
        for i in (0..=max.0).rev() {
            if map.contains_key(&(i, *y)) {
                return (i, *y);
            }
        }
    }
    if direction == &3 {
        for i in (0..=max.1).rev() {
            if map.contains_key(&(*x, i)) {
                return (*x, i);
            }
        }
    }
    unreachable!();
}

pub fn solve2<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let (map, max, movement) = parse(filename);
    let mut direction = 0;
    let directions: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut position = find_outer_edge(&map, &max, &0, &0, &direction);
    let cube_map = get_destinations_cube();
    direction = 3;
    for (turn, steps) in movement.iter() {
        direction = (direction + turn) % directions.len();
        for _ in 0..*steps {
            let velocity = directions[direction];

            //println!("velo {:?}",velocity);
            //println!("ditection  {:?}", direction);
            //println!("pos {:?}",position);
            //println!("");

            let mut target = (
                (position.0 as i64 + velocity.0) as usize,
                (position.1 as i64 + velocity.1) as usize,
            );
            let mut new_direction = direction;
            if !map.contains_key(&(target)) {
                println!("pos {:?}", &(position, direction));
                println!("target {:?}", target);


                (target, new_direction) = *cube_map.get(&(position, direction)).unwrap();
                println!("after transition {:?}", (target, new_direction));
            }
            if map.get(&target).unwrap() == &'#' {
                break;
            }
            direction = new_direction;
            position = target;
        }
    }

    1000 * (position.1 + 1) + 4 * (1 + position.0) + direction
}

fn get_destinations_cube() -> HashMap<((usize, usize), usize), ((usize, usize), usize)> {
    let side_len: usize = 50;

    let mut destinations: HashMap<((usize, usize), usize), ((usize, usize), usize)> =
        HashMap::new();

    // cube has this shape
    //  12
    //  3
    // 45
    // 6
    let offsets = vec![
        (side_len, 0),
        (side_len * 2, 0),
        (side_len, side_len),
        (0, side_len * 2),
        (side_len, side_len * 2),
        (0, side_len * 3),
    ];
    for i in 0..side_len {
        // 1
        // North <->  West 6
        // 1.x = 6.y
        let source = (i + offsets[0].0, offsets[0].1);
        let target = (offsets[5].0, i + offsets[5].1);
        destinations.insert((source, 3), (target, 0));
        destinations.insert((target, 2), (source, 1));

        // West  <-> 4 West
        // 1.y = 4.y.max - 1.y
        let source = (offsets[0].0, i + offsets[0].1);
        let target = (offsets[3].0, offsets[3].1 + side_len - i-1);
        destinations.insert((source, 2), (target, 0));
        destinations.insert((target, 2), (source, 0));

        // 2
        // North <-> 6 South
        // 2.x = 6.x
        let source = (i + offsets[1].0, offsets[1].1);
        let target = (i + offsets[5].0, offsets[5].1 + side_len - 1);
        destinations.insert((source, 3), (target, 3));
        destinations.insert((target, 1), (source, 1));

        // East <-> 5 East
        // 2.y = 5.y.max - 5.y
        let source = (offsets[1].0 + side_len - 1, offsets[1].1 + i);
        let target = (offsets[4].0 + side_len - 1, offsets[4].1 + side_len - i - 1);
        destinations.insert((source, 0), (target, 2));
        destinations.insert((target, 0), (source, 2));
        // South <-> 3 East
        // 2.x = 3.y
        let source = (offsets[1].0 + i, offsets[1].1 + side_len - 1);
        let target = (offsets[2].0 + side_len - 1, offsets[2].1 + i);
        destinations.insert((source, 1), (target, 2));
        destinations.insert((target, 0), (source, 3));

        // 3
        // East solved by 2
        // West <-> 4 North
        // 3.y = 4.x
        let source = (offsets[2].0, i + offsets[2].1);
        let target = (offsets[3].0 + i, offsets[3].1);
        destinations.insert((source, 2), (target, 1));
        destinations.insert((target, 3), (source, 0));
        // 4
        // North solved by 3
        // West solved by 1

        // 5
        // East solved by 2
        // South <-> 6 East
        // 5.x = 6.y
        let source = (offsets[4].0 + i, offsets[4].1 + side_len - 1);
        let target = (offsets[5].0 + side_len - 1, offsets[5].1 + i);
        destinations.insert((source, 1), (target, 2));
        destinations.insert((target, 0), (source, 3));
        // 6
        // West solved by 1
        // South solved by 2
        // East solved by 5
    }

    destinations
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 6032)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 191010)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 2)
    }
}
