use std::collections::HashSet;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u16
where
    P: AsRef<Path>,
{
    let input = parse_data(filename);
    return points_after_fold(input);
}

fn points_after_fold(input: Input) -> u16 {
    if input.folds.len() == 0 {
        println!("{:?}", input.points);

        return input.points.len() as u16;
    }
    return points_after_fold(fold(input));
}

fn fold(mut input: Input) -> Input {
    let fold_to_do = input.folds.first().unwrap();
    if fold_to_do.direction == 'y' {
        input.points = input
            .points
            .iter()
            .map(|point| {
                if point[1] > fold_to_do.position {
                    return [
                        point[0],
                        fold_to_do.position - (point[1] - fold_to_do.position),
                    ];
                }
                return [point[0], point[1]];
            })
            .collect::<Vec<[u32; 2]>>();
    }
    if fold_to_do.direction == 'x' {
        input.points = input
            .points
            .iter()
            .map(|point| {
                if point[0] > fold_to_do.position {
                    return [
                        fold_to_do.position - (point[0] - fold_to_do.position),
                        point[1],
                    ];
                }
                return [point[0], point[1]];
            })
            .collect::<Vec<[u32; 2]>>();
    }
    input.folds.remove(0);
    let mut uniques = HashSet::new();
    input.points.retain(|e| uniques.insert(*e));

    return input;
}

pub fn parse_data<P>(filename: P) -> Input
where
    P: AsRef<Path>,
{
    let mut positions = common::read_lines(filename);
    let blank_line_pos = positions.iter().position(|line| line == "").unwrap();
    let second_part = positions.split_off(blank_line_pos);
    let inputs = second_part
        .iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&String>>();
    let split_positions = common::split_lines(positions, ",");
    return Input {
        points: split_positions
            .iter()
            .map(|position| [position[0].parse().unwrap(), position[1].parse().unwrap()])
            .collect(),
        folds: inputs
            .iter()
            .map(|input| {
                let split = input
                    .split("=")
                    .map(|part| part.to_string())
                    .collect::<Vec<String>>();
                let fold = Fold {
                    direction: input.chars().nth(11).unwrap(),
                    position: split[1].parse().unwrap(),
                };
                return fold;
            })
            .collect(),
    };
}

pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    return 1;
}

#[derive(Debug)]
pub struct Fold {
    direction: char,
    position: u32,
}
impl PartialEq for Fold {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}
#[derive(Debug)]
pub struct Input {
    points: Vec<[u32; 2]>,
    folds: Vec<Fold>,
}
impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points && self.folds == other.folds
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_should_parse_data_correctly() {
        let inputs = super::Input {
            points: vec![
                [6, 10],
                [0, 14],
                [9, 10],
                [0, 3],
                [10, 4],
                [4, 11],
                [6, 0],
                [6, 12],
                [4, 1],
                [0, 13],
                [10, 12],
                [3, 4],
                [3, 0],
                [8, 4],
                [1, 10],
                [2, 14],
                [8, 10],
                [9, 0],
            ],
            folds: vec![
                super::Fold {
                    direction: 'y',
                    position: 7,
                },
                super::Fold {
                    direction: 'x',
                    position: 5,
                },
            ],
        };
        assert_eq!(super::parse_data("././data/sample"), inputs);
    }
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(super::solve1("././data/sample"), 16)
    }
    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(super::solve1("././data/sample2"), 17)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(super::solve1("././data/input1"), 788)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(super::solve1("././data/input2"), 100) // did visulization with python and online compiler
    }
}
