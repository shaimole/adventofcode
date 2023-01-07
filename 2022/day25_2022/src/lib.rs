use std::path::Path;

pub fn solve<P>(filename: P, trips: u8) -> String
where
    P: AsRef<Path>,
{
    let sum = common::read_lines(filename).iter().map(snafu_as_dec).sum();
    desc_as_snafu(sum)
}
fn desc_as_snafu(desc: i128) -> String {
    let mut chars = vec![];
    let mut quotient = desc.clone();
    while quotient != 0 {
        let remainder = (quotient + 2).rem_euclid(5) - 2;
        quotient = (quotient + 2) / 5;
        chars.push(snafu_map_rev(remainder));
    }
    chars.iter().rev().collect()
}

fn snafu_as_dec(snafu: &String) -> i128 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(index, value)| {
            return 5_i128.pow(index as u32) * snafu_map(&value);
        })
        .sum()
}

fn snafu_map(digit: &char) -> i128 {
    match *digit {
        '-' => -1,
        '=' => -2,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => unreachable!(),
    }
}

fn snafu_map_rev(digit: i128) -> char {
    match digit {
        -1 => '-',
        -2 => '=',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve("./data/sample", 1), "2=-1=0")
    }

    #[test]
    fn it_should_decode_snafu() {
        assert_eq!(snafu_as_dec(&"".to_string()), 0);
        assert_eq!(snafu_as_dec(&"1=".to_string()), 3);
    }

    #[test]
    fn it_should_encode_snafu() {
        assert_eq!(desc_as_snafu(314159265), "1121-1110-1=0");
        assert_eq!(desc_as_snafu(2022), "1=11-2");

        assert_eq!(desc_as_snafu(0), "");
        assert_eq!(desc_as_snafu(3), "1=");
    }

    #[test]
    fn it_should_solve_part_1() {
        assert_eq!(solve("./data/input", 1), "20-=0=02=-21=00-02=2")
    }
}
