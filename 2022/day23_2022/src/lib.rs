use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

const CONSIDER: &'static [[(i64, i64); 3]; 4] = &[
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, 1), (1, 0), (1, -1)],
];
const ADJECENT: &'static [(i64, i64); 8] = &[
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
];

fn debug(elfs: &HashSet<(i64, i64)>, min_y: i64, min_x: i64, max_x: i64, max_y: i64) {
    for y in (min_y..=max_y).rev() {
        // print!("{y}");
        for x in min_x..=max_x {
            if !elfs.contains(&(x, y)) {
                print!(".")
            } else {
                print!("#");
            }
        }
        println!("");
    }
}
pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut elfs = parse(filename);
    let (mut max_x, mut max_y, mut min_x, mut min_y) = (i64::MIN, i64::MIN, i64::MAX, i64::MAX);

    for i in 0..10 {
        let mut moves = HashMap::new();

        for elf in elfs.clone() {
            let mut move_to = elf;
            let dont_move = ADJECENT
                .iter()
                .map(|direction| (elf.0 + direction.0, elf.1 + direction.1))
                .filter(|target| elfs.contains(&target))
                .count()
                == 0;
            if dont_move {
                continue;
            }
            for c in 0..4 {
                let dir = (i + c) % 4;
                // println!(" checking direction {dir} for elf {:?}", elf);

                let direction_is_empty = CONSIDER[dir]
                    .iter()
                    .map(|direction| (elf.0 + direction.0, elf.1 + direction.1))
                    .filter(|target| elfs.contains(&target))
                    .count()
                    == 0;
                // println!(
                //     " for elf : {:?}, direction {dir} is empty {direction_is_empty}",
                //     elf
                // );

                if direction_is_empty {
                    match dir {
                        0 => match moves.get(&(elf.0, elf.1 + 1)) {
                            Some((e, count)) => {
                                moves.insert((elf.0, elf.1 + 1), (elf, count + 1));
                            }
                            None => {
                                moves.insert((elf.0, elf.1 + 1), (elf, 1));
                            }
                        },
                        1 => match moves.get(&(elf.0, elf.1 - 1)) {
                            Some((e, count)) => {
                                moves.insert((elf.0, elf.1 - 1), (elf, count + 1));
                            }
                            None => {
                                moves.insert((elf.0, elf.1 - 1), (elf, 1));
                            }
                        },
                        3 => match moves.get(&(elf.0 + 1, elf.1)) {
                            Some((_, count)) => {
                                moves.insert((elf.0 + 1, elf.1), (elf, count + 1));
                            }
                            None => {
                                moves.insert((elf.0 + 1, elf.1), (elf, 1));
                            }
                        },
                        _ => match moves.get(&(elf.0 - 1, elf.1)) {
                            Some((_, count)) => {
                                moves.insert((elf.0 - 1, elf.1), (elf, count + 1));
                            }
                            None => {
                                moves.insert((elf.0 - 1, elf.1), (elf, 1));
                            }
                        },
                    }
                    break;
                }
            }
        }
        moves.iter().for_each(|(target_pos, (elf, count))| {
            if count == &1 {
                elfs.remove(elf);
                elfs.insert(*target_pos);
            }
        });

        println!("==={i}====");
    }
    // println!("{:?}", (elfs, (min_x, max_x, min_y, max_y)));
    elfs.iter().for_each(|elf| {
        max_x = std::cmp::max(elf.0, max_x);
        max_y = std::cmp::max(elf.1, max_y);

        min_x = std::cmp::min(elf.0, min_x);
        min_y = std::cmp::min(elf.1, min_y);
    });
    debug(&elfs, min_y, min_x, max_x, max_y);

    let mut score = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !elfs.contains(&(x, y)) {
                score += 1;
            }
        }
    }
    score
}

fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut elfs = parse(filename);
    let (mut max_x, mut max_y, mut min_x, mut min_y) = (i64::MIN, i64::MIN, i64::MAX, i64::MAX);

    for i in 0..1000 {
        let mut moves = HashMap::new();

        for elf in elfs.clone() {
            let mut move_to = elf;
            let dont_move = ADJECENT
                .iter()
                .map(|direction| (elf.0 + direction.0, elf.1 + direction.1))
                .filter(|target| elfs.contains(&target))
                .count()
                == 0;
            if dont_move {
                continue;
            }
            for c in 0..4 {
                let dir = (i + c) % 4;
                // println!(" checking direction {dir} for elf {:?}", elf);

                let direction_is_empty = CONSIDER[dir]
                    .iter()
                    .map(|direction| (elf.0 + direction.0, elf.1 + direction.1))
                    .filter(|target| elfs.contains(&target))
                    .count()
                    == 0;
                // println!(
                //     " for elf : {:?}, direction {dir} is empty {direction_is_empty}",
                //     elf
                // );

                if direction_is_empty {
                    match dir {
                        0 => match moves.get(&(elf.0, elf.1 + 1)) {
                            Some((e, count)) => {
                                moves.insert((elf.0, elf.1 + 1), (elf, count + 1));
                            }
                            None => {
                                moves.insert((elf.0, elf.1 + 1), (elf, 1));
                            }
                        },
                        1 => match moves.get(&(elf.0, elf.1 - 1)) {
                            Some((e, count)) => {
                                moves.insert((elf.0, elf.1 - 1), (elf, count + 1));
                            }
                            None => {
                                moves.insert((elf.0, elf.1 - 1), (elf, 1));
                            }
                        },
                        3 => match moves.get(&(elf.0 + 1, elf.1)) {
                            Some((_, count)) => {
                                moves.insert((elf.0 + 1, elf.1), (elf, count + 1));
                            }
                            None => {
                                moves.insert((elf.0 + 1, elf.1), (elf, 1));
                            }
                        },
                        _ => match moves.get(&(elf.0 - 1, elf.1)) {
                            Some((_, count)) => {
                                moves.insert((elf.0 - 1, elf.1), (elf, count + 1));
                            }
                            None => {
                                moves.insert((elf.0 - 1, elf.1), (elf, 1));
                            }
                        },
                    }
                    break;
                }
            }
        }
        if moves
            .iter()
            .filter(|(target_pos, (elf, count))| count == &1)
            .count()
            == 0
        {
            return i as u32 + 1;
        }
        moves.iter().for_each(|(target_pos, (elf, count))| {
            if count == &1 {
                elfs.remove(elf);
                elfs.insert(*target_pos);
            }
        });
    }
    unreachable!();
}

fn parse<P>(filename: P) -> HashSet<(i64, i64)>
where
    P: AsRef<Path>,
{
    let mut map = HashSet::new();
    common::read_lines(filename)
        .iter()
        .rev()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    map.insert((x as i64, y as i64));
                }
            });
        });
    map
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 110)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 20)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 4162)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 986)
    }
}
