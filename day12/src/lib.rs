use common;
use std::collections::HashMap;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u16 
 where P: AsRef<Path>,{
    let caves = parse_cave(filename);
    return traverse(&"start".to_string(), &"end".to_string(), &vec![], caves);
}

fn parse_cave<P>(filename: P) -> HashMap<String, Vec<String>> 
where P: AsRef<Path>, {
    let lines = common::read_lines(filename);
    let connections_between_caves = common::split_lines(lines, "-");
    let mut caves = init_edges(HashMap::new(), &connections_between_caves);
    caves = init_edges_reverse(caves, &connections_between_caves);
    return caves;
}

fn init_edges(mut caves: HashMap<String, Vec<String>>, connections: &Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    for connection in connections {
        let origin = connection[0].to_owned();
        let destination = connection[1].to_owned();
        match caves.get_mut(&origin) {
            Some(connections) => {
                connections.push(destination);
            },
            None => {
                caves.insert(origin, vec![destination]);
            }
        }
    }
    return caves;
}

fn init_edges_reverse(mut caves: HashMap<String, Vec<String>>, connections: &Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    for connection in connections {
        let origin = connection[0].to_owned();
        let destination = connection[1].to_owned();
        match caves.get_mut(&destination) {
            Some(connections) => {
                connections.push(origin);
            },
            None => {
                caves.insert(destination, vec![origin]);
            }
        }
    }
    return caves;
}

fn traverse(current_cave: &String, target_cave: &String, visited : &Vec<String>, caves: HashMap<String, Vec<String>>) -> u16 {
    if current_cave == target_cave {
        return 1;
    }
    let mut new_visited = visited.to_owned();
    let is_big_cave = current_cave.chars().nth(0).unwrap().is_uppercase();
    if !is_big_cave {
        new_visited.push(current_cave.to_owned());
    }
    let mut total_paths = 0;
    match caves.get(current_cave) {
            Some(connections) => {
                for connection in connections.to_owned() {
                    if visited.contains(&connection) {
                        continue;
                    }
                    total_paths += traverse(&connection, target_cave, &new_visited,caves.to_owned());
                }
            },
            None => {
                return total_paths;
            }
        }
    
    return total_paths;
}

fn traverse_2(current_cave: &String, target_cave: &String, visited_once : &Vec<String>, caves: HashMap<String, Vec<String>>, visited_twice: &Vec<String>) -> u32 {
    if current_cave == target_cave {
        return 1;
    }
    let mut new_visited_once = visited_once.to_owned();
    let mut new_visited_twice = visited_twice.to_owned();

    let is_big_cave = current_cave.chars().nth(0).unwrap().is_uppercase();

    if !is_big_cave && new_visited_once.contains(current_cave){
        new_visited_twice.push(current_cave.to_owned());
    }
    if !is_big_cave {
        new_visited_once.push(current_cave.to_owned());
    }


    let mut total_paths = 0;
    match caves.get(current_cave) {
            Some(connections) => {
                for connection in connections.to_owned() {
                    if visited_twice.contains(&connection) {
                        continue;
                    }
                    if ( visited_once.contains(&connection) && new_visited_twice.len() > 0 ) || connection == "start"  {
                        continue;
                    }
                    total_paths += traverse_2(&connection, target_cave, &new_visited_once, caves.to_owned(), &new_visited_twice);
                    
                }
            },
            None => {
                return total_paths;
            }
        }
    
    return total_paths;
}


pub fn solve2<P>(filename: P) -> u32 
where P: AsRef<Path>,{
    let caves = parse_cave(filename);
    return traverse_2(&"start".to_string(), &"end".to_string(), &vec![], caves, &vec![]);
}
 

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_parse_data_correctly() {
        let cave = super::parse_cave("././data/test");
        match cave.get("start") {
            Some(connections) => {
                assert_eq!(connections.contains(&String::from("A")),true);
                assert_eq!(connections.contains(&String::from("b")),true);
            },
            None => {
                assert_eq!(true,false)
            }
        }
        match cave.get("A") {
            Some(connections) => {
                assert_eq!(connections.contains(&String::from("start")),true);
                assert_eq!(connections.contains(&String::from("end")),true);
                assert_eq!(connections.contains(&String::from("b")),true);
                assert_eq!(connections.contains(&String::from("c")),true);
            },
            None => {
                assert_eq!(true,false)
            }
        }
    }
    #[test]
    fn it_should_solve_first_test_case_part2() {
        assert_eq!(super::solve2("././data/test"),36)
    }

    #[test]
    fn it_should_solve_second_test_case_part2() {
        assert_eq!(super::solve2("././data/test2"),103)
    }

    #[test]
    fn it_should_solve_third_test_case_part2() {
        assert_eq!(super::solve2("././data/test3"),3509)
    }


    #[test]
    fn it_should_solve_first_test_case() {
        assert_eq!(super::solve1("././data/test"),10)
    }

    #[test]
    fn it_should_solve_second_test_case() {
        assert_eq!(super::solve1("././data/test2"),19)
    }

    #[test]
    fn it_should_solve_third_test_case() {
        assert_eq!(super::solve1("././data/test3"),226)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(super::solve1("././data/sample1"),3679)
    }
    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(super::solve2("././data/sample1"),107395)
    }
}
