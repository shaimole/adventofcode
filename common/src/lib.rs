
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// do the whole reading of lines from file in one line
// see also: https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// why not pass the results
pub fn lines_to_int(file_content: io::Result<io::Lines<io::BufReader<File>>>) -> Vec<i32> {
    let mut converted = Vec::new();
    if let Ok(lines) = file_content{
        for line in lines {
            if let Ok(line) = line {
                converted.push(line.parse().unwrap());
            }
        }
    }
    return converted;
}

// don't pass in result
pub fn split_lines(file_content: io::Result<io::Lines<io::BufReader<File>>>, delimiter: &str) -> Vec<Vec<String>> {
    let mut converted = Vec::new();
    if let Ok(lines) = file_content {
        for line in lines {
            if let Ok(line) = line {
                let split_line = line.split(delimiter).map(|part| part.to_string()).collect();
                converted.push(split_line);
            }
        }
    }
    return converted;
}

pub fn string_binary_to_u32(zeros_and_ones: String) -> u32 {
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
        if let Ok(file_content) = super::read_lines("././data/sampleInt") {
            for line in file_content {
                if let Ok(line) = line {
                   assert_eq!(line, "1");
                }
            }
        }
    }
    #[test]   
    fn it_converts_read_lines_to_int() {
            let file_content= super::read_lines("././data/sampleInt");
            let lines = super::lines_to_int(file_content);
            assert_eq!(lines.len(),6);
            for line in lines {
                   assert_eq!(line, 1);
            }

    }

    #[test]
    fn it_coverts_read_lines_and_splits_lines_by_space() {
        let file_content= super::read_lines("././data/sampleIntStr");
            let lines = super::split_lines(file_content," ");
            assert_eq!(lines.len(),6);
    }

    #[test] 
    fn it_converts_string_to_u32() {
        let converted = super::string_binary_to_u32("0101".to_string());
        assert_eq!(converted,5);
    }
        
}
