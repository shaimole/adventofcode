use util;
use std::path::Path;

pub fn solve1<P>(filename: P) -> i32 
where P: AsRef<Path>,{
//    let draws = read_result_numbers(filename);
//    let boards = read_boards(filename);
//    for draw in draws {
//        for board in boards {
//             board.update(draw);
//        }
//    }
   return 1;
}

pub fn solve2<P>(filename: P) -> i32 
where P: AsRef<Path>,{
    return 1;
}


fn read_result_numbers<P>(filename: P) -> Vec<u16> 
where P: AsRef<Path>, {
    let lines = util::read_lines(filename);
    let str_vec = util::split_lines(lines,",")[0].to_vec();
    let u_vec: Vec<u16> = str_vec.iter().map(|x| x.parse::<u16>().unwrap()).collect();
    return u_vec;
}

struct BingoBoard {
    horizontal_rows: Vec<Vec<u16>>,
    vertical_rows: Vec<Vec<u16>>,
}

impl BingoBoard {

    pub fn score(self) -> u16 {
        let mut score: u16 = 0;
        let iter = self.horizontal_rows.iter();
        for row in iter {
            let row_score: u16 = row.iter().sum();
            score = row_score + score;
        }
        return score;
    }

    pub fn update(mut self, score_number: u16) -> BingoBoard {
        let mut new_rows_horizontal = Vec::new(); 
        let mut new_rows_vertical = Vec::new();
        for row in self.horizontal_rows {
            let mut new_row = Vec::new();
            for number in row {
                if number != score_number {
                    new_row.push(number);
                } 
            }
            new_rows_horizontal.push(new_row);
        }
        for row in self.vertical_rows {
            let mut new_row = Vec::new();
            for number in row {
                if number != score_number {
                    new_row.push(number);
                } 
            }
            new_rows_vertical.push(new_row);
        }
        self.vertical_rows = new_rows_vertical;
        self.horizontal_rows = new_rows_horizontal;
        return self;
    }

    pub fn isBingo(&self) -> bool {
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

fn read_boards<P>(filename: P) -> Vec<BingoBoard>
where P: AsRef<Path>, {
    let lines = util::read_lines(filename);
    let  mut str_vec = util::split_lines(lines," ").to_vec();
    str_vec.remove(0); // remove result numbers
    let mut bingo_boards = Vec::new();
    let mut rows = Vec::new();
    for line in str_vec {
        if line.len() < 2 {
            continue;
        }
        let numbers: Vec<u16> = line.iter().filter(|x| x != &"").map(|x| x.parse::<u16>().unwrap()).collect();
        rows.push(numbers);
        if rows.len() == 5 {
            let mut vertical_rows: Vec<Vec<u16>> =Vec::new();
            for _i in 0..rows.len() {
                let empty = Vec::new();
                vertical_rows.push(empty);
            }
            for i in 0..rows.len() {
                for n in 0..rows[i].len() {
                    vertical_rows[n].push(rows[i][n]);
                }
            }
            rows = rows.clone();
            let board = BingoBoard {
                horizontal_rows: rows,
                vertical_rows: vertical_rows,
            };
            bingo_boards.push(board);
            rows = Vec::new();
        }
    }
    return bingo_boards;
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_result_numbers() {
        let result = super::read_result_numbers("././data/test");
        assert_eq!(result.len(),27);
    }

    #[test]
    fn it_should_read_the_boards() {
         let result = super::read_boards("././data/test");
        assert_eq!(result.len(),3);
        assert_eq!(result[0].horizontal_rows.len(),5);
        assert_eq!(result[0].vertical_rows.len(),5);
    }

    // #[test]
    // fn it_should_solve_testdata_for_part_1() {
    //     assert_eq!(super::solve1("././data/test"),4512);
    // }

    #[test]
    fn board_should_update_correctly() {
        let horizontal_rows = vec![vec![1,2,3,4,5,6]];
        let mut vertical_rows = Vec::new();
        vertical_rows.push(vec![1]);
        vertical_rows.push(vec![2]);
        vertical_rows.push(vec![3]);
        vertical_rows.push(vec![4]);
        vertical_rows.push(vec![5]);
        vertical_rows.push(vec![6]);
        let board = super::BingoBoard {
            horizontal_rows: horizontal_rows,
            vertical_rows: vertical_rows,
        };
        let score = board.update(1).score();
        let is_bingo = board.isBingo();
        assert_eq!(score,20);
        assert_eq!(is_bingo,true);
    }
    // #[test]    
    // fn it_should_solve_testdata_for_part_2() {
    //     assert_eq!(super::solve2("././data/test"),900);
    // }
}
