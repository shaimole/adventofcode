use common;
use std::path::Path;


pub fn solve1<P>(filename : P,  mut stacks: Vec<Vec<char>>) -> String 
where P: AsRef<Path>, {
    let instructions = parse(filename);

    for crane_move in instructions {
        for n in 0..crane_move.amount {
            let cargo = stacks[crane_move.from as usize -1].pop().unwrap();
            stacks[crane_move.to as usize -1].push(cargo);
         }
    }
    stacks.iter().map(|stack| stack[stack.len()-1 ]).collect()
    
}

pub fn solve2<P>(filename : P,  mut stacks: Vec<Vec<char>>) -> String 
where P: AsRef<Path>, {
    let instructions = parse(filename);
    for crane_move in instructions {
            let len = stacks[crane_move.from as usize -1].len();
            let to_move: Vec<_> = stacks[crane_move.from as usize -1].drain(len- crane_move.amount as usize..).collect();
            for cargo in to_move {
                stacks[crane_move.to as usize -1].push(cargo)
            }
    }
    stacks.iter().map(|stack| stack[stack.len()-1 ]).collect()
}


pub fn sample_input() -> Vec<Vec<char>> {
    vec![
        vec![
            'Z','N'
        ],
        vec![
            'M','C','D'
        ],
        vec![
           'P' 
        ]
    ]
}
pub fn input() -> Vec<Vec<char>> {
    vec![
        vec![
            'G','T','R','W'
        ],
        vec![
            'G','C','H','P','M','S','V','W'
        ],
        vec![
           'C','L','T','S','G','M' 
        ],
        vec![
            'J','H','D','M','W','R','F'
        ],
        vec![
            'P','Q','L','H','S','W','F','J',
        ],
        vec![
            'P','J','D','N','F','M','S'
        ],
        vec![
            'Z','B','D','F','G','C','S','J'
        ],
        vec![
            'R','T','B'
        ],
        vec![
            'H','N','W','L','C'
        ]
    ]
}

pub fn parse<P>(filename : P) -> Vec<CraneMove> 
where P: AsRef<Path>, {
    let lines = common::split_lines(common::read_lines(filename)," ");
    lines.iter().map( |line| CraneMove { from: line[3].parse().unwrap(), to: line[5].parse().unwrap(), amount: line[1].parse().unwrap() }).collect()
}

pub struct CraneMove {
    from: u32,
    to: u32,
    amount: u32
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve1("./data/sample", sample_input()),"CMZ")
    }

    #[test]
    fn it_should_parse_samples() {
        let crane_move = vec![CraneMove{ from: 111, to:22, amount:1111}];
        assert_eq!(parse("./data/sample2")[0].to,crane_move[0].to);
        assert_eq!(parse("./data/sample2")[0].amount,crane_move[0].amount);
        assert_eq!(parse("./data/sample2")[0].from,crane_move[0].from);
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample",super::sample_input()),"MCD")
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve1("./data/input", input()),"JCMHLVGMG")
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input", input()),"LVMRWSSPZ")
    }

}
