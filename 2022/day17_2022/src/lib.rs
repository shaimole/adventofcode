use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

pub fn solve<P>(filename: P, mut piece_count: u64) -> u64
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
    let mut i = 0;
    let mut prev_i = 0;
    let mut height_mod = 0;
    let mut skip = true;
    let mut prev_i_peak = 0;
    while i <= piece_count {
        let pi = i as usize % 5;
        let piece = &pieces[pi];
        current_peak = find_peak(&the_pit, current_peak);
        let offset_y = current_peak + 4;
        let mut offset: (u64, u64) = (offset_x, offset_y);
        loop {
            let x_move = x_moves[current_move % total_moves];
            if current_move % total_moves == 0 && current_move != 0 && skip {
                if prev_i_peak != 0 {
                    println!(" current step{:?}", i);
                    println!(" left  steps {:?}", piece_count - i);
                    println!(" height diff {:?}", current_peak - prev_i_peak);
                    println!(" piece diff {:?}", i - prev_i);
                    let skip_pieces = (piece_count - i) / (i - prev_i) * (i - prev_i);
                    println!(" steps to skip  {:?}", skip_pieces);

                    height_mod = (current_peak - prev_i_peak)
                        * ((piece_count - i) / (i - prev_i) * (i - prev_i) / (i - prev_i));
                    println!("height to add {:?}", height_mod);

                    i += skip_pieces;
                    println!(" current step{:?}", i);
                    skip = false;
                }
                prev_i_peak = current_peak;
                prev_i = i;
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
        i += 1;
    }
    current_peak + height_mod
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
fn map_floor(current: &HashSet<(u64, u64)>, offset: u64) -> Vec<Vec<i32>> {
    let mut floor = vec![];
    for x in 1..8 {
        let mut line = vec![];
        for y in 0..10 {
            if current.contains(&(x, offset - y)) {
                line.push(y as i32);
            }
        }
        floor.push(line);
    }
    floor
}
fn print(current: &HashSet<(u64, u64)>, offset: u64) {
    for y in (offset..offset + 100).rev() {
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
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input", 2022), 3127)
    }

    #[test]
    fn it_should_solve_part_1_ext() {
        assert_eq!(solve("./data/input", 7000), 10810)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve("./data/input", 1000000000000), 1542941176480)
    }
}
