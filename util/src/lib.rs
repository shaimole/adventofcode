
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// fn main() {
//     // File hosts must exist in current path before this produces output
//     if let Ok(lines) = read_lines("./hosts") {
//         // Consumes the iterator, returns an (Optional) String
//         for line in lines {
//             if let Ok(ip) = line {
//                 println!("{}", ip);
//             }
//         }
//     }
// }

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file() -> i32 {
    return 3;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_reads_lines() {
        if let Ok(file_content) = super::read_lines("../data/sample") {
            for line in file_content {
                if let Ok(line) = line {
                   assert_eq!(line, "1");
                }
            }
        }
 
        
    }
}
