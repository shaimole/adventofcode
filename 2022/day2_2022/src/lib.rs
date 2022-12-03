use common;
use std::path::Path;
use std::collections::HashMap;

pub fn solve1<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    return solve(get_scoring_1(),common::read_lines(filename));
}

pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    return solve(get_scoring_2(),common::read_lines(filename));
}

fn solve(scoring: HashMap<String,u32>, rounds: Vec<String>) -> u32 {
    return rounds.iter().map(|round| scoring.get(round).unwrap()).sum();
}

fn get_scoring_1() -> HashMap<String,u32> {
    return HashMap::from([
        ("A X".to_string(),1 + 3),
        ("A Y".to_string(),2 + 6),
        ("A Z".to_string(),3 + 0),
        ("B X".to_string(),1 + 0),
        ("B Y".to_string(),2 + 3),
        ("B Z".to_string(),3 + 6),
        ("C X".to_string(),1 + 6),
        ("C Y".to_string(),2 + 0),
        ("C Z".to_string(),3 + 3),
    ]);
}

fn get_scoring_2() -> HashMap<String,u32> {
    return HashMap::from([
        ("A X".to_string(),3 + 0),
        ("A Y".to_string(),1 + 3),
        ("A Z".to_string(),2 + 6),
        ("B X".to_string(),1 + 0),
        ("B Y".to_string(),2 + 3),
        ("B Z".to_string(),3 + 6),
        ("C X".to_string(),2 + 0),
        ("C Y".to_string(),3 + 3),
        ("C Z".to_string(),1 + 6),
    ]);
}



#[cfg(test)]
mod tests {

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(super::solve1("./data/sample"),15)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(super::solve2("./data/sample"),12)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(super::solve1("./data/input"),13484)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(super::solve2("./data/input"),13433)
    }

}