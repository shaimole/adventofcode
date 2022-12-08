use std::collections::HashSet;
use std::path::Path;

fn parse<P>(filename : P ) -> Vec<Vec<u32>> 
where P: AsRef<Path>, {
    common::split_lines(common::read_lines(filename),&"")
    .iter().map(
        |line| 
            line.iter().filter(|string| !string.is_empty()).map(|line| line.parse().unwrap()).collect()
    ).collect()
}


pub fn solve<P>(filename : P) -> usize 
where P: AsRef<Path>, {

    let grid = parse(filename);
    let mut visible_horizontal = HashSet::new();
    let mut visible_vertial: HashSet<Vec<usize>> = HashSet::new();
    let mut current_height:i32;

    for i in 0..grid.len() {
        current_height = -1;
        for j in 0..grid[i].len() {
            let tree_height = grid[i][j];
                if   current_height == -1 || tree_height > current_height as u32{
                    visible_horizontal.insert(vec![i,j]);
                    current_height = tree_height as i32;
                }
        }
        current_height = -1;
        for j in (0..grid[i].len()).rev() {
            let tree_height = grid[i][j];
                if   current_height == -1 || tree_height > current_height as u32 {
                    visible_horizontal.insert(vec![i,j]);
                    current_height = tree_height as i32;
                }
        }
    }
    
    for i in 0..grid[0].len() {
        current_height = -1;
        for j in 0..grid.len() {
            let tree_height = grid[j][i];
                if  current_height == -1 ||tree_height > current_height as u32 {
                    visible_vertial.insert(vec![j,i]);
                    current_height = tree_height as i32;
                }
        }
    }
    for i in 0..grid[0].len() {
        current_height = -1;
        for j in (0..grid.len()).rev() {
            let tree_height = grid[j][i];
                if  current_height == -1 || tree_height > current_height as u32 {
                    visible_vertial.insert(vec![j,i]);
                    current_height = tree_height as i32;
                }
        }
    }
    visible_horizontal.extend(visible_vertial);
    visible_horizontal.len()
}

pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    let grid = parse(filename);
    let mut score = 0;
     for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let tree_score = scenic_score(&grid, i, j);
            if tree_score > score {
                score = tree_score;
            }
        }
    }
    score
}

pub fn scenic_score(grid: &Vec<Vec<u32>>,i:usize, j:usize) -> u32 {
    let mut score_left = 0;
    let mut score_right = 0;
    let mut score_down = 0;
    let mut score_up = 0;
    let current_height = grid[i][j] as i32;
    // down
    for ii in i+1..grid.len() {
        score_down += 1;
        if grid[ii][j] >= current_height as u32{
            break;
        }        
    }

    // up
    for im in (0..i).rev() {
        score_up += 1;
        if grid[im][j] >= current_height as u32 {
            break;
        }   
    }

    // right
    for jj in j + 1..grid[i].len() {
        score_right += 1;
        if  grid[i][jj] >= current_height as u32{
            break;
        }        
    }

    // left    
    for jj in (0..j).rev() {
        score_left += 1;
        if  grid[i][jj] >= current_height as u32{
            break;
        }        
    }
    score_down * score_up * score_left * score_right as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"),21)
    }

    #[test]
    fn it_should_calculate_scenic_score() {
        let map = parse("./data/sample");
        assert_eq!(scenic_score(&map,1,2),4);
        assert_eq!(scenic_score(&map,0,0),0);
        assert_eq!(scenic_score(&map,4,4),0);
        assert_eq!(scenic_score(&map,3,2),8);

    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"),8)
    }
    #[test]
    fn it_should_solve_part_1() {
         assert_eq!(solve("./data/input"),1789)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"),314820)
    }

}
