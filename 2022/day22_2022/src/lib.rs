use std::collections::HashMap;
use std::path::Path;

fn print(map: &HashMap<((i64, i64), char)>, max: &(i64, i64)) {
    for y in (0..max.1).rev() {
        for x in 0..max.0 {
            if !map.contains_key(&(x, y)) {
                print!(" ");
            }
            print!(map.get(&x,y));
        }
        println!("");
    }
}

pub fn parse<P>(filename: P) ->  (HashMap<((i64, i64), char))>, (i64,i64), Vec<char>)>
where
    P: AsRef<Path>,
{
    let mut max_x = 0;
    let mut map: HashMap<((i64, i64), char))> = HashMap::new();
    let lines = common::read_lines(filename);
    for y in 0..lines.len() {
        if line.as_str() == "" {
            break;
        }
        for (x,c) in 0..line[y].chars().enumerate(){
            if c == ' ' {
                continue;
            }
            map.insert((x,y),c);
            max_x = std::cmp::max(x,max_x);
        }
    }
    (map,(max_x,lines.len() as i64 - 2),lines[lines.len()-1].chars().collect()
     )
}

pub fn solve<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    let (map,max,movement) = parse(filename);
    print(&map,&max);
    0
}

pub fn solve2<P>(filename: P) -> i128
where
    P: AsRef<Path>,
{
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 6032)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample") - 1)
    }
    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), -1)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), -1)
    }
}
