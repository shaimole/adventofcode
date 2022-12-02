use common;
use std::path::Path;
use std::collections::HashMap;



pub fn solve1<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    let instructions = common::split_lines(common::read_lines(filename)," ");
    let map = get_map();
    let single_points_map = get_map_for_values();
    return instructions.iter().map( |pair| map.get(&pair[1]).unwrap().get(&pair[0]).unwrap()  + single_points_map.get(&pair[1]).unwrap() ).sum();
}

pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
    let instructions = common::split_lines(common::read_lines(filename)," ");
    let map = get_map_2();
    let values = get_map_for_values_2();

    let mut points = 0;
    for pair in instructions {
          let winpoints = values.get(&pair[1]).unwrap();
          let choicepoints = map.get(&pair[0]).unwrap().get(winpoints).unwrap();
          points += winpoints + choicepoints;
    }
    
    points
}
fn get_map_for_values() ->HashMap<String,u32> {
    let mut scoring = HashMap::new();
     scoring.insert(
        "X".to_string(),
        1,
    );
    scoring.insert(
        "Y".to_string(),
        2,
    );
    scoring.insert(
        "Z".to_string(),
        3,
    );
    
    scoring
}

fn get_map_for_values_2() ->HashMap<String,u32> {
    let mut scoring = HashMap::new();
     scoring.insert(
        "X".to_string(),
        0,
    );
    scoring.insert(
        "Y".to_string(),
        3,
    );
    scoring.insert(
        "Z".to_string(),
        6,
    );
    
    scoring
}

fn get_map() -> HashMap<String,HashMap<String,u32>> {
    let mut scoring = HashMap::new();
    let mut x_points = HashMap::new();
    x_points.insert("A".to_string(), 3);
    x_points.insert("B".to_string(), 0);
    x_points.insert("C".to_string(), 6);

    let mut y_points = HashMap::new();
    y_points.insert("A".to_string(), 6);
    y_points.insert("B".to_string(), 3);
    y_points.insert("C".to_string(), 0);

    let mut z_points = HashMap::new();
    z_points.insert("A".to_string(), 0);
    z_points.insert("B".to_string(), 6);
    z_points.insert("C".to_string(), 3);

    scoring.insert(
        "X".to_string(),
        x_points,
    );
    scoring.insert(
        "Y".to_string(),
        y_points,
    );
    scoring.insert(
        "Z".to_string(),
        z_points,
    );
    return scoring
}

fn get_map_2() -> HashMap<String,HashMap<u32,u32>> {
    let mut scoring = HashMap::new();
    let mut x_points = HashMap::new();
    x_points.insert(0, 3);
    x_points.insert(3, 1);
    x_points.insert(6, 2);

    let mut y_points = HashMap::new();
    y_points.insert(0, 1);
    y_points.insert(3, 2);
    y_points.insert(6, 3);

    let mut z_points = HashMap::new();
    z_points.insert(0, 2);
    z_points.insert(3, 3);
    z_points.insert(6, 1);

    scoring.insert(
        "A".to_string(),
        x_points,
    );
    scoring.insert(
        "B".to_string(),
        y_points,
    );
    scoring.insert(
        "C".to_string(),
        z_points,
    );
    return scoring
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