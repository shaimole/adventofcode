use common;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u16
where
    P: AsRef<Path>,
{
    return 1;
}

pub fn parseData<P>(filename: P) -> Input
where
    P: AsRef<Path>,
{
    let mut positions = common::read_lines(filename);
    let blank_line_pos = positions.iter().position(|line| line == "").unwrap();
    let second_part = positions.split_off(blank_line_pos);
    let inputs = second_part.iter().filter(|line| !line.is_empty()).collect::<Vec<&String>>();
    let split_positions = common::split_lines(positions, ",");
    return Input {
        points: split_positions.iter()
        .map(|position| 
            [
                position[0].parse().unwrap(),
                position[1].parse().unwrap()
            ]
        ).collect(),
        folds: inputs.iter()
        .map(|input| {
            return Fold {
             direction: input.chars().nth(11).unwrap(),
             position: input.chars().nth(13).unwrap().to_digit(10).unwrap()   
            }
        }
        ).collect(),
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
    position: u32
}
impl PartialEq for Fold {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}
#[derive(Debug)]
pub struct Input {
    points: Vec<[u8; 2]>,
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

        assert_eq!(super::parseData("././data/sample"), inputs);
    }
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(super::solve1("././data/sample"), 17)
    }
    fn it_should_solve_part_1() {}

    #[test]
    fn it_should_solve_part_2() {}
}
