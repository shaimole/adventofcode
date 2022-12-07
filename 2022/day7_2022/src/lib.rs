use std::collections::HashMap;
use std::path::Path;

fn parse(instructions : &Vec<String>) -> HashMap<Vec<&str>,u32>  {
    let mut sizes: HashMap<Vec<&str>,u32> = HashMap::new();
    let mut current_dirs: Vec<&str> = vec![];
    sizes.insert(vec!["/"],0);
    instructions.iter()
        .map( |line| line.split(" ").collect() )
        .for_each(|instruction: Vec<&str>| {
            let mut key = current_dirs.clone();
            match instruction[0] {
                "dir" =>  {
                    key.push(instruction[1]);
                     if sizes.get(&key) == None {
                            sizes.insert(key,0);
                        }
                },
                "cd" => {
                    if instruction[1] == ".." {
                        current_dirs.pop();
                    }else {
                        if sizes.get(&key) == None {
                            sizes.insert(key,0);
                        }
                        current_dirs.push(instruction[1]);
                    }
                },
                _ => { 
                    let mut key = vec![];
                    current_dirs.iter().for_each(
                        
                        |dir| {
                         key.push(*dir);
                         *sizes.get_mut(&key).unwrap() += instruction[0].parse::<u32>().unwrap();
                        }
                    );
                },
            };
        }
    );
    sizes
}


pub fn solve<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    let instructions = common::read_lines(filename).iter()
        .filter(|line| line != &"$ ls")
        .map(|line| line.replace("$ ","")).collect();
    let sizes = parse(&instructions);
    let mut result = 0;
    sizes.iter().filter(|&(_, v)| *v <= 100000).for_each(|(_,v)| result += v);
    result
}

pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    let instructions = common::read_lines(filename).iter()
        .filter(|line| line != &"$ ls")
        .map(|line| line.replace("$ ","")).collect();
    let sizes = parse(&instructions);
    let max_disk_space = 70000000;
    let disk_size_needed = 30000000;
    let occupied_space = sizes.get(&vec!["/"]).unwrap();
    let disk_free = max_disk_space - occupied_space;
    let mut minimum_to_delte = u32::MAX;
    sizes.iter().filter(|&(_, v)| *v >= disk_size_needed - disk_free).for_each(
        |(_,v)| 
        {
            if v < &minimum_to_delte {
                minimum_to_delte = *v
            }
        }
    );
    minimum_to_delte
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"),95437)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"),24933642)
    }
    #[test]
    fn it_should_solve_part_1() {
         assert_eq!(solve("./data/input"),1084134)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"),6183184)
    }

}
