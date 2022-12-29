use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

pub fn solve<P>(filename: P,piece_count: u64) -> u64
where
    P: AsRef<Path>,
{
    let mut the_pit = create_tetris_walls();
    let pieces: Vec<Vec<(u64, u64)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut current_peak = 0;
    let offset_x = 3;
    let lines = common::read_lines(filename);
    let x_moves: Vec<char> = lines[0].chars().collect();
    let total_moves = x_moves.len();
    let mut current_move = 0;
    let mut loops = HashMap::new();
    for i in 0..=piece_count {
        let piece = &pieces[i % 5];
        current_peak = find_peak(&the_pit, current_peak);
        let offset_y = current_peak + 4;
        let mut offset: (u64, u64) = (offset_x, offset_y);
        loop {
            let x_move = x_moves[current_move % total_moves];
            if (current_move % total_moves == 0 && current_move != 0) {
                let floor = map_floor(&the_pit, current_peak);
                if loops.contains_key(&(i % 5, floor.clone())) {
                    let prev: &(usize,u64) =
                        loops.get(&(i%5,floor.clone())).unwrap();

                    println!(
                        " loop deteced beginning at{:?} height  {:?},blocks {:?}",
                        
                        prev.0,
                        current_peak-prev.1,
                        i -prev.0
                    );
                } else {
                    loops.insert(
                        ((i % 5, map_floor(&the_pit, current_peak))),
                        (i, current_peak),
                    );
                }
            }
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
    }
    current_peak
}

fn find_peak(current: &HashSet<(u64, u64)>, last_peak: u64) -> u64 {
    for y in (last_peak..=last_peak + 4).rev() {
        for x in 1..8 {
            if current.contains(&(x, y)) {
                return y;
            }
        }
    }
    unreachable!();
}
fn map_floor(current: &HashSet<(u64, u64)>, offset: u64) -> Vec<i32> {
    let mut floor = vec![];
    for x in 1..8 {
        for y in 0..100 {
            if current.contains(&(x, offset - y)) {
                floor.push(y as i32);
                break;
            }
        }
    }
    floor
}
fn print(current: &HashSet<(u64, u64)>, offset: u64) {
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
fn create_tetris_walls() -> HashSet<(u64, u64)> {
    let mut map: HashSet<(u64, u64)> = HashSet::new();
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
        assert_eq!(solve("./data/sample",2022), 3068)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve("./data/sample",50000), 3127)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input",2022), 3127)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve("./data/input",50000), 500)
    }
}
