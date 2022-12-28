use std::collections::HashSet;
use std::path::Path;

pub fn solve<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut the_pit = create_tetris_walls();
    let pieces: Vec<Vec<(u32, u32)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut current_peak = 0;
    let offset_x = 3;
    let lines = common::read_lines(filename);
    let mut x_moves = lines[0].chars();
    let total_moves = x_moves.clone().count();
    let mut current_move = 0;
    for i in 0..=2022 {
        let piece = &pieces[i % 5];
        current_peak = find_peak(&the_pit, current_peak);
        println!("{:?}", current_peak);
        let offset_y = current_peak + 4;
        let mut offset: (u32, u32) = (offset_x, offset_y);
        loop {
            println!("{:?}", current_move % total_moves);
            let x_move = lines[0].chars().nth(current_move % total_moves).unwrap();
            current_move += 1;
            match x_move {
                '>' => {
                    if piece
                        .iter()
                        .map(|(x, y)| (x + offset.0 + 1, y + offset.1))
                        .filter(|p| the_pit.contains(p))
                        .count()
                        == 0
                    {
                        offset.0 += 1;
                    }
                }
                '<' => {
                    if piece
                        .iter()
                        .map(|(x, y)| (x + offset.0 - 1, y + offset.1))
                        .filter(|p| the_pit.contains(p))
                        .count()
                        == 0
                    {
                        offset.0 -= 1;
                    }
                }
                _ => unreachable!(),
            }
            if piece
                .iter()
                .map(|(x, y)| (x + offset.0, y + offset.1 - 1))
                .filter(|p| the_pit.contains(p))
                .count()
                > 0
            {
                the_pit.extend(piece.iter().map(|(x, y)| (x + offset.0, y + offset.1)));
                break;
            }
            offset.1 -= 1;
        }
        print(&the_pit, 0);
    }
    current_peak
}

fn find_peak(current: &HashSet<(u32, u32)>, last_peak: u32) -> u32 {
    for y in (last_peak..=last_peak + 4).rev() {
        for x in 1..8 {
            if current.contains(&(x, y)) {
                return y;
            }
        }
    }
    unreachable!();
}
fn print(current: &HashSet<(u32, u32)>, offset: u32) {
    for y in (offset..offset + 10).rev() {
        for x in 0..9 {
            if current.contains(&(x, y)) {
                print!("#");
            } else {
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
fn create_tetris_walls() -> HashSet<(u32, u32)> {
    let mut map: HashSet<(u32, u32)> = HashSet::new();
    for i in 0..9 {
        map.insert((i, 0));
    }
    for i in 0..100000 {
        map.insert((0, i));
        map.insert((8, i));
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
        assert_eq!(solve2("./data/sample"), 3127)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input"), 3127)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"), 500)
    }
}
