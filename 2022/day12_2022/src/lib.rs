use std::path::Path;
use std::collections::HashSet;


pub fn solve<P>(filename: P) -> i32
where
    P: AsRef<Path>
{
    let map = get_height_map(filename);
    let start = find_start(&map);
    travel(0, HashSet::new(), vec![start], &map, i32::MAX)
}

fn travel(steps: i32, mut visited: HashSet<(usize,usize)>, current_set: Vec<(usize,usize)>, map: &Vec<Vec<String>>, max_steps: i32 ) -> i32 {
    if steps >= max_steps {
        return steps;
    }
    let mut next_set = vec![];
    for current_pos in current_set {
        let adjecent = get_neighbours(current_pos, map.len() - 1, map[0].len() -1);
        let current_elevation = get_elevation(&map[current_pos.0][current_pos.1]);
        for cell in adjecent {
            let point = &map[cell.0][cell.1];
            if !visited.contains(&cell) &&  get_elevation(&point) - current_elevation <= 1 {
                visited.insert(cell);
                if point == "E" {
                    return steps + 1;
                }
                next_set.push(cell);
            }
        }
    }
    travel(steps + 1,visited,next_set,map, max_steps)
}

fn get_height_map<P>(filename: P) -> Vec<Vec<String>>
where P: AsRef<Path> {
    common::split_lines_no_empty_strings(common::read_lines(filename), "")
}

fn find_start(map: &Vec<Vec<String>>) -> (usize,usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == "S" {
                return (i,j)
            }
        }
    }
    unreachable!()
}

fn get_neighbours(current: (usize,usize), m: usize, n: usize) -> Vec<(usize,usize)> {
    let mut adjecent: Vec<(usize,usize)> = vec![];
    let i = current.0;
    let j = current.1;
    if i > 0 {
        adjecent.push((i-1,j));
    }
    if j > 0 {
        adjecent.push((i,j-1));
    }
    if i < m {
        adjecent.push((i+1,j));
    }
    if j < n {
        adjecent.push((i,j+1));
    }
    adjecent
}

fn get_elevation(character: &String) -> i32 {
    if character == "S" {
        return 'a' as i32 -96;
    }
    if character == "E" {
        return 'z' as i32 - 96;
    }
    character.chars().next().unwrap() as i32 - 96
}


pub fn solve2<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let map = get_height_map(filename);
    let mut min_steps = i32::MAX;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if  ["S".to_string(),"a".to_string()].contains(&map[i][j]) {
                let distance = travel(0, HashSet::new(), vec![(i,j)], &map, min_steps);
                min_steps = std::cmp::min(distance,min_steps);
            }
        }
    }
    min_steps
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_find_starting_points() {
        let map_sample = get_height_map("./data/sample");
        let map_input = get_height_map("./data/input");
        assert_eq!(find_start(&map_sample),(0,0));
        assert_eq!(find_start(&map_input),(20,0));
    }

    #[test]
    fn it_should_find_neighbours() {
        assert_eq!(get_neighbours((0,0), 1, 1).sort_unstable(), vec![(0,1),(1,0)].sort_unstable());
        assert_eq!(get_neighbours((2,2), 3, 3).sort_unstable(), vec![(2,1),(2,3),(1,2),(3,2)].sort_unstable());
    }
    #[test]
    fn it_should_get_correct_elevation() {
        assert_eq!(get_elevation(&"a".to_string()), get_elevation(&"S".to_string()));
        assert_eq!(get_elevation(&"a".to_string()), get_elevation(&"b".to_string()) -1);
    }
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 31)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve2("./data/sample"), 29)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 504)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), -1000)
    }
}
