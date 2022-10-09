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
    return Input {
        points: vec![],
        folds: vec![],
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
    position: u8,
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
                [0, 15],
                [9, 11],
                [0, 3],
                [10, 5],
                [4, 12],
                [6, 0],
                [6, 13],
                [4, 1],
                [0, 14],
                [10, 13],
                [3, 4],
                [3, 0],
                [8, 4],
                [1, 11],
                [2, 15],
                [8, 11],
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
