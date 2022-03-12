
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


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

#[cfg(test)]
mod tests {
    #[test]
    fn it_reads_lines() {
        if let Ok(file_content) = super::read_lines("././data/sample") {
            for line in file_content {
                if let Ok(line) = line {
                   assert_eq!(line, "1");
                }
            }
        }
    }
    #[test]   
    fn it_converts_read_lines_to_int() {
            let file_content= super::read_lines("././data/sample");
            let lines = super::lines_to_int(file_content);
            assert_eq!(lines.len(),6);
            for line in lines {
                   assert_eq!(line, 1);
            }

    }
        
}
