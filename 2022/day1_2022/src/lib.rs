use common;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut calories = get_calorie_sums(filename);
    calories.sort_unstable();
    return *calories.last().unwrap();
}

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut calories = get_calorie_sums(filename);
    calories.sort_unstable();
    return calories.iter().rev().take(3).sum();
}

fn get_calorie_sums<P>(filename: P) -> Vec<u32>
where
    P: AsRef<Path>,
{
    let calories = common::read_lines(filename);
    let mut current_sum: u32 = 0;
    let mut sums = vec![];
    for calorie in calories {
        if calorie.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
            continue;
        }
        let u_calorie: u32 = calorie.parse().unwrap();
        current_sum += u_calorie;
    }
    sums
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(super::solve1("./data/input"), 72602)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(super::solve2("./data/input"), 207410)
    }
}
