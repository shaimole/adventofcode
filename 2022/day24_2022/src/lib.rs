use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::Path;

const ADJECENT: &'static [(i64, i64); 4] = &[(1, 0), (0, -1), (-1, 0), (0, 1)];
const MOVES: &'static [(i64, i64); 5] = &[(0, 0), (1, 0), (0, -1), (-1, 0), (0, 1)];

fn debug(
    blizzards: &HashMap<(i64, i64), char>,
    size_x: &i64,
    size_y: &i64,
    start: &(i64, i64),
    finish: &(i64, i64),
    pos: &(i64, i64),
) {
    for y in (-1..=*size_y).rev() {
        for x in -1..=*size_x {
            if &(x, y) == pos {
                print!("E")
            } else if &(x, y) == start {
                print!("S")
            } else if &(x, y) == finish {
                print!("F")
            } else if !blizzards.contains_key(&(x, y)) {
                print!(".")
            } else {
                let symbol = blizzards.get(&(x, y)).unwrap();
                print!("{symbol}");
            }
        }
        println!("");
    }
}
pub fn solve<P>(filename: P, trips: u8) -> i64
where
    P: AsRef<Path>,
{
    let (blizzards_start, size_x, size_y, mut start, mut end) = parse(filename);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut trip = 1;
    println!("{:?}", (start, end));
    queue.push_back((start, 0));
    while let Some((pos, time)) = queue.pop_front() {
        if !visited.insert((pos, time % (size_x * size_y))) {
            continue;
        }
        if pos == end {
            if trip == trips {
                return time - 1;
            } else {
                queue = VecDeque::new();
                visited = HashSet::new();
                let tmp = end.clone();
                end = start.clone();
                start = tmp;

                queue.push_back((pos, time));

                trip += 1;
                continue;
            }
        }
        let moved = move_blizzards(&blizzards_start, &size_x, &size_y, &time);
        MOVES
            .iter()
            .map(|(x, y)| (pos.0 + x, pos.1 + y))
            .filter(|(x, y)| !moved.contains_key(&(*x, *y)))
            .filter(|(x, y)| (x >= &0 && y >= &0) || (*x, *y) == start || (*x, *y) == end)
            .filter(|(x, y)| (x < &size_x && y < &size_y) || (*x, *y) == start || (*x, *y) == end)
            .for_each(|option| queue.push_back((option, time + 1)));
    }

    unreachable!();
}

fn move_blizzards(
    blizzards: &HashMap<(i64, i64), char>,
    size_x: &i64,
    size_y: &i64,
    time: &i64,
) -> HashMap<(i64, i64), char> {
    blizzards
        .iter()
        .map(|((x, y), c)| {
            let direction = blizzard_direction(c);
            let mut n_direction = (
                (
                    (x + direction.0 * *time).rem_euclid(*size_x),
                    (y + direction.1 * *time).rem_euclid(*size_y),
                ),
                *c,
            );
            if n_direction.0 .0 < 0 {
                n_direction.0 .0 += size_x - 1;
            }
            if n_direction.0 .1 < 0 {
                n_direction.0 .1 += size_y;
            }
            return n_direction;
        })
        .collect()
}
fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}

fn blizzard_direction(symbol: &char) -> (i64, i64) {
    match *symbol {
        '>' => ADJECENT[0],
        'v' => ADJECENT[1],
        '<' => ADJECENT[2],
        '^' => ADJECENT[3],
        _ => unreachable!(),
    }
}

fn parse<P>(filename: P) -> (HashMap<(i64, i64), char>, i64, i64, (i64, i64), (i64, i64))
where
    P: AsRef<Path>,
{
    let mut blizzards = HashMap::new();
    let lines = common::read_lines(filename);
    let max_y = lines.iter().count() as i64 - 2;
    let max_x = lines[0].chars().count() as i64 - 2;
    let finish = (max_x - 1, -1);
    let start = (0, max_y);
    lines.iter().rev().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if "><^v".chars().filter(|bc| bc == &c).count() > 0 {
                blizzards.insert((x as i64 - 1, y as i64 - 1), c);
            }
        });
    });
    (blizzards, max_x, max_y, start, finish)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample", 1), 18)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve("./data/sample", 3), 54)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input", 1), 326)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve("./data/input", 3), 976)
    }
}
