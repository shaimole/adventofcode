use common;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u16
where
    P: AsRef<Path>,
{
    let draws = read_draws(&filename);
    let mut boards = parse_boards(&filename);
    for draw in draws {
        for n in 0..boards.len() {
            boards[n] = boards[n].update(draw);
            if boards[n].is_bingo() {
                return boards[n].score() * draw;
            }
        }
    }
    return 1;
}

fn read_draws<P>(filename: P) -> Vec<u16>
where
    P: AsRef<Path>,
{
    let lines = common::read_lines(filename);
    let str_vec = common::split_lines(lines, ",")[0].to_vec();
    let u_vec: Vec<u16> = str_vec.iter().map(|x| x.parse::<u16>().unwrap()).collect();
    return u_vec;
}

fn parse_boards<P>(filename: P) -> Vec<BingoBoard>
where
    P: AsRef<Path>,
{
    const BOARD_SIZE: usize = 5;
    let boards_input = get_boards_data(filename);
    let mut bingo_boards = vec![];
    for board_rows in boards_input {
        let mut vertical_rows: Vec<Vec<u16>> = vec![vec![]; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            for n in 0..BOARD_SIZE {
                vertical_rows[n].push(board_rows[i][n]);
            }
        }
        let board = BingoBoard {
            horizontal_rows: board_rows,
            vertical_rows: vertical_rows,
        };
        bingo_boards.push(board);
    }
    return bingo_boards;
}

fn get_boards_data<P>(filename: P) -> Vec<Vec<Vec<u16>>>
where
    P: AsRef<Path>,
{
    let mut lines = common::read_lines(filename);
    lines.remove(0); // remove drawn numbers line
    let split_lines = common::split_lines(lines, " ").to_vec();
    let lines_no_blanks = split_lines
        .iter()
        .filter(|line| line.len() > 2)
        .collect::<Vec<_>>();
    let mut boards = vec![];
    let mut single_board_rows = vec![];
    for line in lines_no_blanks {
        let numbers_of_row: Vec<u16> = line
            .iter()
            .filter(|number| number != &"")
            .map(|number| number.parse::<u16>().unwrap())
            .collect();
        single_board_rows.push(numbers_of_row);

        if single_board_rows.len() == 5 {
            boards.push(single_board_rows);
            single_board_rows = vec![];
        }
    }
    return boards;
}

pub fn solve2<P>(filename: P) -> u16
where
    P: AsRef<Path>,
{
    let draws = read_draws(&filename);
    let mut boards = parse_boards(&filename);

    let mut bingos = [0; 10000];
    for draw in draws {
        for n in 0..boards.len() {
            boards[n] = boards[n].update(draw);
            if boards[n].is_bingo() {
                bingos[n] = bingos[n] + 1;
                let mut bingo_count = 0;
                for bingo in bingos {
                    if bingo >= 1 {
                        bingo_count = bingo_count + 1;
                    }
                }
                if bingo_count == boards.len() {
                    return boards[n].score() * draw;
                }
            }
        }
    }
    return 1;
}

struct BingoBoard {
    horizontal_rows: Vec<Vec<u16>>,
    vertical_rows: Vec<Vec<u16>>,
}

impl BingoBoard {
    pub fn score(&self) -> u16 {
        let mut score: u16 = 0;
        let iter = self.horizontal_rows.iter();
        for row in iter {
            let row_score: u16 = row.iter().sum();
            score = row_score + score;
        }
        return score;
    }

    pub fn update(&self, score_number: u16) -> BingoBoard {
        let mut new_rows_horizontal = Vec::new();
        let mut new_rows_vertical = Vec::new();
        for row in &self.horizontal_rows {
            let mut new_row = Vec::new();
            for number in row {
                if number != &score_number {
                    new_row.push(*number);
                }
            }
            new_rows_horizontal.push(new_row);
        }
        for row in &self.vertical_rows {
            let mut new_row = Vec::new();
            for number in row {
                if number != &score_number {
                    new_row.push(*number);
                }
            }
            new_rows_vertical.push(new_row);
        }
        return BingoBoard {
            vertical_rows: new_rows_vertical,
            horizontal_rows: new_rows_horizontal,
        };
    }

    pub fn is_bingo(&self) -> bool {
        for row in &self.horizontal_rows {
            if row.len() == 0 {
                return true;
            }
        }
        for row in &self.vertical_rows {
            if row.len() == 0 {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_result_numbers() {
        let result = super::read_draws("././data/test");
        assert_eq!(result.len(), 27);
    }

    #[test]
    fn it_should_read_the_boards() {
        let result = super::parse_boards("././data/test");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].horizontal_rows.len(), 5);
        assert_eq!(result[0].vertical_rows.len(), 5);
    }

    #[test]
    fn it_should_solve_testdata_for_part_1() {
        assert_eq!(super::solve1("././data/test"), 4512);
    }

    #[test]
    fn it_should_solve_testdata_for_part_2() {
        assert_eq!(super::solve2("././data/test"), 1924);
    }

    #[test]
    fn it_should_solve1() {
        assert_eq!(super::solve1("././data/sample1"), 54275)
    }

    #[test]
    fn it_should_solve2() {
        assert_eq!(super::solve2("././data/sample1"), 13158)
    }

    #[test]
    fn board_should_update_correctly() {
        let horizontal_rows = vec![vec![1, 2, 3, 4, 5, 6]];
        let mut vertical_rows = Vec::new();
        vertical_rows.push(vec![1]);
        vertical_rows.push(vec![2]);
        vertical_rows.push(vec![3]);
        vertical_rows.push(vec![4]);
        vertical_rows.push(vec![5]);
        vertical_rows.push(vec![6]);
        let mut board = super::BingoBoard {
            horizontal_rows: horizontal_rows,
            vertical_rows: vertical_rows,
        };
        board = board.update(1);
        assert_eq!(board.is_bingo(), true);
        assert_eq!(board.score(), 20);
    }
}
