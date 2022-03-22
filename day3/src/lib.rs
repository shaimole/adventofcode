use common;
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
   let total_bits = get_amount_of_bits(&data);
   let converted_data = parse(&data);
   return co2_rating(&converted_data, total_bits as usize) * oxigen_rating(&converted_data, total_bits as usize);
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
            gamma_rate = gamma_rate + 2u32.pow(value_of_bit as u32);
        }
    }
    return gamma_rate;
}

fn epsilon_rate(gamma_rate: u32, total_bits: u32) -> u32 {
    return !fill_unset_bits(gamma_rate, total_bits);
}

fn fill_unset_bits(number: u32, set_bits: u32) -> u32 {
  let mut filled = number;
  for n in 0..32 {
        if n > set_bits - 1 {
            filled = filled + 2u32.pow(n);
        }
    }
    return filled;
}

fn oxigen_rating(data: &Vec<u32>, total_bits: usize) -> u32{
    let mut current_set = data.clone();   
    for n in (0..total_bits).rev() {
        let amount_of_datapoints = current_set.len() as f32;
        let totals = get_total_amount_of_set_bits(&current_set);
        if totals[31-n] as f32 >= amount_of_datapoints /2.0{
            current_set = filter_set_by_bit_set(current_set, n)
        }else {
            current_set = filter_set_by_bit_unset(current_set, n)
        }
        if current_set.len() == 1 {
            break;
        }
    }

    return current_set[0];
}


fn co2_rating(data: &Vec<u32>, total_bits: usize) -> u32{
    let mut current_set = data.clone();   
    for n in (0..total_bits).rev() {
        let amount_of_datapoints = current_set.len() as f32;
        let totals = get_total_amount_of_set_bits(&current_set);
        if totals[31-n] as f32 >= amount_of_datapoints /2.0{
            current_set = filter_set_by_bit_unset(current_set, n)
        }else {
            current_set = filter_set_by_bit_set(current_set, n)
        }
        if current_set.len() == 1 {
            break;
        }
    }
    return current_set[0];
}

fn filter_set_by_bit_set(data: Vec<u32>, position: usize) -> Vec<u32>{
    let mut filtered = Vec::new();
    for binary in data {
        if common::get_bit(&binary,position as usize) {
            filtered.push(binary);
        }
    }
    return filtered;
}

fn filter_set_by_bit_unset(data: Vec<u32>, position: usize) -> Vec<u32>{
    let mut filtered = Vec::new();
    for binary in data {
        if !common::get_bit(&binary,position as usize) {
            filtered.push(binary);
        }
    }
    return filtered;
}

fn get_total_amount_of_set_bits(data: &Vec<u32>) -> [u32;32] {
  let mut totals: [u32; 32] = [0;32];
    for n  in 0..32 as usize {
        for binary in data {
            if common::get_bit(binary, n) {
                totals[n] = totals[n] +1;
            }
        }
    }
    totals.reverse();
    return totals;
}




fn read_data<P>(filename: P) -> Vec<Vec<String>>
where P: AsRef<Path>, {
    let lines = common::read_lines(filename);
    let split = common::split_lines(lines," ");
    return split;
}


fn parse(data: &Vec<Vec<String>>) -> Vec<u32> {
    let mut converted = Vec::new();
    for sequence in data {
        converted.push(common::string_binary_to_u32(sequence[0].clone()));
    }
    return converted;
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
        let lines = common::read_lines("././data/test");
        let split = common::split_lines(lines," ");
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
    #[test]
    fn it_solves1 () {
        let result = super::solve1("././data/sample1");
        assert_eq!(result,738234);
    }

    #[test]
    fn it_calcs_oxigen() {
        let data = super::read_data("././data/test");
        let data = super::parse(&data);
        let oxigen_rating = super::oxigen_rating(&data,5);
        assert_eq!(oxigen_rating,23);
    }

    #[test]
    fn it_cals_co2() {
        let data = super::read_data("././data/test");
        let data = super::parse(&data);
        let co2_rating = super::co2_rating(&data,5);
        assert_eq!(co2_rating,10);
    }
}
