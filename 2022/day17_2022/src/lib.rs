use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

pub fn solve<P>(filename: P, piece_count: u64) -> u64
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
    let mut skip_done = true;
    let mut i = 0;
    while i <=piece_count {
        let pi = i as usize % 5;
        let piece = &pieces[pi];
        current_peak = find_peak(&the_pit, current_peak);
        let offset_y = current_peak + 4;
        let mut offset: (u64, u64) = (offset_x, offset_y);
        loop {
            let x_move = x_moves[current_move % total_moves];
            if (current_move % total_moves == 0 && current_move != 0 && !skip_done) {
                let floor = map_floor(&the_pit, current_peak);
                if loops.contains_key(&(pi, floor.clone())) {
                    let prev: &(usize, u64) = loops.get(&(pi, floor.clone())).unwrap();

                    println!(
                        " loop deteced beginning at{:?} height  {:?},blocks {:?}",
                        prev.0,
                        current_peak - prev.1,
                        i - prev.0 as u64
                    );
                    let pieces_remaining = piece_count -i;
                    let loops_fitting_in_remaining = pieces_remaining/ (i -prev.0 as u64);
                    i += loops_fitting_in_remaining *(i -prev.0 as u64);
                    let space_diff = loops_fitting_in_remaining * (current_peak - prev.1);
                    current_peak += space_diff;
                    for x in 1..8 {
                        for y in 0..100 {
                            if the_pit.contains(&(x, current_peak -space_diff - y)) {
                                the_pit.insert((x,current_peak - y));
                            }
                        }
                    }
                    skip_done = true;
                        println!("{:?}",i);
                        println!("{:?}",current_peak);
                        println!("{:?}",piece_count);
    for wi in 0..1000000 {
        the_pit.insert((0, wi+current_peak -100));
        the_pit.insert((8, wi+current_peak -100));
    }
                    print(&the_pit,current_peak-30);
                } else if(!skip_done) {
                    loops.insert(
                        ((pi, map_floor(&the_pit, current_peak))),
                        (i as usize, current_peak),
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
        i += 1;
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
    fn it_should_solve_sample1() {
        assert_eq!(solve("./data/sample", 2022), 3068)
    }

    #[test]
    fn it_should_solve_sample2() {
        assert_eq!(solve("./data/sample", 1000000000000), 1514285714288)
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input", 2022), 3127)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve("./data/input", 1000000000000), 500)
    }
}
