use std::path::Path;
use std::collections::HashSet;


pub fn solve<P>(filename: P) -> i32
where
    P: AsRef<Path>
{
    let map = get_height_map(filename);
    let start = find_start(&map);
    let mut visited = HashSet::new();
    visited.insert(start);
    travel(0, visited, start, map)
}

fn travel(steps: i32, visited: HashSet<(usize,usize)>,current_pos: (usize,usize), map: Vec<Vec<String>> ) -> i32 {
    if map[current_pos.0][current_pos.1] == "E" {
            return steps;
    }
    let mut steps_to_goal = i32::MAX;
    let adjecent = get_neighbours(current_pos, map.len() - 1, map[0].len() -1);
    let current_elevation = get_elevation(&map[current_pos.0][current_pos.1]);
    for cell in adjecent {
        if !visited.contains(&cell) && current_elevation - get_elevation(&map[cell.0][cell.1]) >= -1 {
            let mut child_visit = visited.clone();
            child_visit.insert(cell);
            let child_map =  map.clone();
            let steps_to_goal_for_path = travel(steps + 1, child_visit, cell, child_map);
            if steps_to_goal_for_path < steps_to_goal {
                steps_to_goal = steps_to_goal_for_path
            }
            
        }
    }
    steps_to_goal
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


pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    1
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
        // assert_eq!(solve2("./data/sample"), 1)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 15020)
    }

    #[test]
    fn it_should_solve_part_2() {
        // assert_eq!(solve2("./data/input"), 1)
    }
}
