
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let reader =  io::BufReader::new(file);
    return reader.lines().collect::<Result<_, _>>().unwrap();
}

pub fn lines_to_int(lines: Vec<String>) -> Vec<i32> {
    return lines.iter().map(|line| line.parse().unwrap()).collect();
}

pub fn split_lines(lines: Vec<String>, delimiter: &str) -> Vec<Vec<String>> {
    return lines.iter().map(|line| line.split(delimiter)
        .map(|part| part.to_string()).collect())
    .collect();
}

pub fn get_bit(number: &u32, position: usize) -> bool{
    return number & (1 << position) != 0
}

pub fn string_binary_to_u32(zeros_and_ones: &String) -> u32 {
    let mut binary_digit =  zeros_and_ones.chars().count();
    let mut real_num: u32 = 0;
    for c in zeros_and_ones.chars() { 
        let mut temp_var = 2u32.pow(binary_digit.try_into().unwrap());
        temp_var /= 2;
        if c == '1'{
            real_num += temp_var;
        }
        binary_digit -= 1;
    }
    return real_num;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_reads_lines() {
       let file_content = super::read_lines("././data/sampleInt");
        for line in file_content {
            assert_eq!(line, "1");
        }

    }

    #[test]   
    fn it_converts_read_lines_to_int() {
            let lines = super::read_lines("././data/sampleInt");
            let int_lines = super::lines_to_int(lines);
            assert_eq!(int_lines.len(),6);
            for line in int_lines {
                assert_eq!(line, 1);
            }

    }

    #[test]
    fn it_coverts_read_lines_and_splits_lines_by_space() {
        let lines = super::read_lines("././data/sampleIntStr");
        let split = super::split_lines(lines," ");
        assert_eq!(split.len(),6);
        assert_eq!(split[0],vec!["1","a"])
    }

    #[test] 
    fn it_converts_string_to_u32() {
        let string = "0101".to_string();
        let converted = super::string_binary_to_u32(&string);
        assert_eq!(converted,5);
    }

    #[test]
    fn it_gets_a_specific_bit() {
        let number = 7;

        let mut is_set = super::get_bit(&number, 0);
        assert_eq!(is_set,true);

        is_set = super::get_bit(&number, 1);
        assert_eq!(is_set,true);

        let is_set = super::get_bit(&number, 2);
        assert_eq!(is_set,true);
    }
        
}
