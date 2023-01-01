use common;
use std::collections::HashSet;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    common::read_lines(filename)
        .iter()
        .map(calculate_backpack_score)
        .sum()
}

fn calculate_backpack_score(backpack: &String) -> u32 {
    let matches: HashSet<_> = backpack.chars().take(backpack.len() / 2).collect();

    for c in backpack.chars().skip(backpack.len() / 2) {
        if matches.contains(&c) {
            return char_to_score(c);
        }
    }
    unreachable!()
}

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    common::read_lines(filename)
        .chunks(3)
        .map(calculate_badges_score)
        .sum()
}

fn calculate_badges_score(backpacks: &[String]) -> u32 {
    let matches_1: HashSet<_> = backpacks[0].chars().collect();
    let matches_2: HashSet<_> = backpacks[1].chars().collect();

    for c in backpacks[2].chars() {
        if matches_1.contains(&c) && matches_2.contains(&c) {
            return char_to_score(c);
        }
    }
    unreachable!()
}

pub fn char_to_score(c: char) -> u32 {
    let mut delta = 96;
    if c.is_uppercase() {
        delta = 64 - 26;
    }
    return c as u32 - delta;
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_score_character() {
        assert_eq!(super::char_to_score('p'), 16);
        assert_eq!(super::char_to_score('L'), 38);
        assert_eq!(super::char_to_score('P'), 42);
        assert_eq!(super::char_to_score('v'), 22);
        assert_eq!(super::char_to_score('t'), 20);
        assert_eq!(super::char_to_score('s'), 19);
    }
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(super::solve1("./data/sample"), 157)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(super::solve2("./data/sample"), 70)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(super::solve1("./data/input"), 7727)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(super::solve2("./data/input"), 2609)
    }
}
