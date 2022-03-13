use util;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u32 
where P: AsRef<Path>,{
   let data = read_data(filename);
   let total_bits = get_amount_of_bits(&data);
   let converted_data = parse(&data);
   return power_consumption(&converted_data, total_bits);
}

pub fn solve2<P>(filename: P) -> u32 
where P: AsRef<Path>,{
   let data = read_data(filename);
   return 2; 
}

fn power_consumption(data: &Vec<u32>, total_bits: u32) -> u32 {
    let totals = get_total_amount_of_set_bits(data);
    let gamma_rate = gamma_rate(totals,data.len());
    let epsilon_rate = epsilon_rate(gamma_rate, total_bits);
    return gamma_rate * epsilon_rate;
}

fn gamma_rate(bits: [u32;32], length: usize) -> u32 {
    let mut gamma_rate:u32 = 0;
    for n in (0..32).rev() {
        let value_of_bit = 32- n -1;
        if bits[n] as usize > length /2 {
            println!("{:?}", 1);
            gamma_rate = gamma_rate + 2u32.pow(value_of_bit as u32);
        }
        println!("{:?}", 0);
    }
    return gamma_rate;
}
fn get_total_amount_of_set_bits(data: &Vec<u32>) -> [u32;32] {
  let mut totals: [u32; 32] = [0;32];
    for n  in 0..32 as usize {
        for binary in data {
            if get_bit(binary, n) {
                totals[n] = totals[n] +1;
            }
        }
    }
    totals.reverse();
    println!("{:?}", totals);

    return totals;
}


fn get_bit(number: &u32, position: usize) -> bool{
    if number & (1 << position) != 0 {
        return true;
    }
    return false;
}


fn read_data<P>(filename: P) -> Vec<Vec<String>>
where P: AsRef<Path>, {
    let lines = util::read_lines(filename);
    let split = util::split_lines(lines," ");
    return split;
}


fn parse(data: &Vec<Vec<String>>) -> Vec<u32> {
    let mut converted = Vec::new();
    for sequence in data {
        converted.push(util::string_binary_to_u32(sequence[0].clone()));
    }
    return converted;
}

fn epsilon_rate(gamma_rate: u32, total_bits: u32) -> u32 {
    let mut epsilon_rate = gamma_rate;
    for n in 0..32 {
        if n > total_bits - 1 {
            epsilon_rate = epsilon_rate + 2u32.pow(n);
        }
    }
    return !epsilon_rate;
}

fn get_amount_of_bits(data: &Vec<Vec<String>>) -> u32 {
    return data[0][0].len() as u32;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_testfile() {
        let data = super::read_data("././data/test");
        assert_eq!(data.len(),12);
    }

    #[test]
    fn it_should_calculate_the_gamma_rate() {
        let data = super::read_data("././data/test");
        let data = super::parse(&data);
        let totals = super::get_total_amount_of_set_bits(&data);
        let gamma_rate = super::gamma_rate(totals, data.len());
        assert_eq!(gamma_rate,22);
    }

    #[test]
    fn it_should_calculate_number_of_total_ones_correctly() {
        let data = super::read_data("././data/test");
        let data = super::parse(&data);
        let totals = super::get_total_amount_of_set_bits(&data);
        let expected: [u32;32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 5, 8, 7, 5];
        assert_eq!(expected,totals); 
    }

     #[test]    
    fn it_calculates_length_of_inputs() {
        let lines = util::read_lines("././data/test");
        let split = util::split_lines(lines," ");
        let set_length = super::get_amount_of_bits(&split);
        assert_eq!(set_length,5)
    }

    #[test]    
    fn it_calculate_epsilon_rate() {
        assert_eq!(super::epsilon_rate(334 as u32, 9) ,177);
    }

    #[test]
    fn it_calculate_power_consumption() {
        let data = super::read_data("././data/test");
        let data = super::parse(&data);
        assert_eq!(super::power_consumption(&data, 5),198);
    }
}
