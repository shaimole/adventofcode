use util;
use std::path::Path;

pub fn solve1<P>(filename: P) -> i32 
where P: AsRef<Path>,{
   return 1; 
}

pub fn solve2<P>(filename: P) -> i32 
where P: AsRef<Path>,{
   return 2; 
}

fn most_common_bit(data: Vec<Vec<String>>, row: usize) -> i32 {
    let mut amount = 0;
    for sequence in data {
        let to_check = sequence[0].chars().nth(row).unwrap();
        if to_check == '1' {
            amount = amount + 1;
            continue;
        }
        amount = amount -1;
    }
    return if amount > 0 {1} else {0};
}

fn gamma_rate(data: Vec<Vec<String>>) -> i32 {
    return 1;
}


fn read_data<P>(filename: P) -> Vec<u32>
where P: AsRef<Path>, {
    let lines = util::read_lines(filename);
    let split = util::split_lines(lines," ");
    return parse(split);
}


fn parse(data: Vec<Vec<String>>) -> Vec<u32> {
    let mut converted = Vec::new();
    for sequence in data {
        let mut binary_digit =  sequence[0].chars().count();
        let mut real_num: u32 = 0;
        for c in sequence[0].chars() { 
            let mut temp_var = 2u32.pow(binary_digit.try_into().unwrap());
            temp_var /= 2;
            if c == '1'{
                real_num += temp_var;
            }
            binary_digit -= 1;
            }
        converted.push(real_num);
    }
    return converted;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_testfile() {
        let data = super::read_data("././data/test");
        assert_eq!(data.len(),12);
    }

    // #[test]
    // fn it_should_solve_find_most_common_bit_1() {
    //     let data = super::read_data("././data/test");
    //     assert_eq!(super::most_common_bit(data,0),1);

    // }
    // #[test]
    // fn it_should_calculate_the_gamma_rate() {
    //     let data = super:read_data
    // }

    // #[test]    
    // fn it_should_solve_testdata_for_part_2() {
    //     assert_eq!(super::solve2("././data/test"),900);
    // }
}
