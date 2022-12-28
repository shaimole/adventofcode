use std::path::Path;
use std::collections::HashSet;


pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>
{
    let mut the_pit = create_tetris_walls();
    let pieces: Vec<Vec<(u32,u32)>> = vec![
        vec![(0,0),(1,0),(2,0),(3,0)],
        vec![(0,1),(1,0),(1,1),(1,2),(2,1)],
        vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        vec![(0,0),(0,1),(0,2),(0,3)],
        vec![(0,0),(0,1),(1,0),(1,1)]
    ];
        let offset: (u32,u32) =(3,4);
        the_pit.extend(pieces[4].iter().map( |(x,y)| (x + offset.0, y + offset.1)));
    print(&the_pit,0);
    0
}

fn print(current: &HashSet<(u32,u32)>, offset: u32) {
    for y in (offset..offset+10).rev() {
        for x in 0..9{
            if current.contains(&(x,y)) {
                print!("#");
            }else {
                print!(".");
            }
        }
        println!("");
    }
}
pub fn solve2<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    0
}
fn create_tetris_walls() -> HashSet<(u32,u32)> {
    let mut map: HashSet<(u32,u32)> = HashSet::new();
    for i in 0..9 {
        map.insert((i,0));
    }
    for i in 0..100000 {
        map.insert((0,i));
        map.insert((8,i));
    }
    map
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample"), 3068)
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
        assert_eq!(solve2("./data/input"), 500)
    }
}
