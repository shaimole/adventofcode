use common;
use std::path::Path;

pub fn solve1<P>(filename: P) -> u32 
where P: AsRef<Path>,{
   let binaries_set = parse_data(filename);
   return power_consumption(binaries_set);
}

fn parse_data<P>(filename: P) -> BinariesSet
where P: AsRef<Path>, {
    let input_lines = common::read_lines(filename);
    return BinariesSet {
        binaries : input_lines.iter().map(|line| common::string_binary_to_u32(line)).collect(),
        binaries_length: input_lines[0].len()
    }
}

struct BinariesSet {
    binaries: Vec<u32>,
    binaries_length: usize
}

fn power_consumption(binaries_set: BinariesSet) -> u32 {
    let totals = get_total_amount_of_set_bits(&binaries_set.binaries);
    let gamma_rate = gamma_rate(totals, binaries_set.binaries.len());
    let epsilon_rate = epsilon_rate(gamma_rate, binaries_set.binaries_length);
    return gamma_rate * epsilon_rate;
}


fn get_total_amount_of_set_bits(data: &Vec<u32>) -> [u32;32] {
    let mut totals: [u32; 32] = [0;32];
      for n  in 0..32 as usize {
          for binary in data {
              if common::get_bit(binary, n) {
                  totals[n] += 1;
              }
          }
      }
      totals.reverse();
      return totals;
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

fn epsilon_rate(mut gamma_rate: u32, binaries_length: usize) -> u32 {
  for n in 0..32 {
        let bit_already_used_by_number = n <  binaries_length;
        if bit_already_used_by_number {
            continue
        }
        gamma_rate += 2u32.pow(n as u32);
    }
    return !gamma_rate;
}

pub fn solve2<P>(filename: P) -> u32 
where P: AsRef<Path>,{
   let binaries_set = parse_data(filename);
   return co2_rating(&binaries_set) * oxigen_rating(&binaries_set);
}


fn oxigen_rating(binaries_set: &BinariesSet) -> u32{
    let mut current_set = binaries_set.binaries.clone();   
    for n in (0..binaries_set.binaries_length).rev() {
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


fn co2_rating(binaries_set: &BinariesSet) -> u32{
    let mut current_set = binaries_set.binaries.clone();   
    for n in (0..binaries_set.binaries_length).rev() {
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



#[cfg(test)]
mod tests {
    #[test]
    fn it_should_read_the_testfile() {
        let data = super::parse_data("././data/test");
        assert_eq!(data.binaries.len(),12);
    }

    #[test]
    fn it_should_calculate_the_gamma_rate() {
        let data = super::parse_data("././data/test");
        let totals = super::get_total_amount_of_set_bits(&data.binaries);
        let gamma_rate = super::gamma_rate(totals, data.binaries.len());
        assert_eq!(gamma_rate,22);
    }

    #[test]
    fn it_should_calculate_number_of_total_ones_correctly() {
        let data = super::parse_data("././data/test");
        let totals = super::get_total_amount_of_set_bits(&data.binaries);
        let expected: [u32;32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 5, 8, 7, 5];
        assert_eq!(expected,totals); 
    }

     #[test]    
    fn it_calculates_length_of_inputs() {
        let data = super::parse_data("././data/test");
        assert_eq!(data.binaries_length,5)
    }

    #[test]    
    fn it_calculate_epsilon_rate() {
        assert_eq!(super::epsilon_rate(334 as u32, 9) ,177);
    }

    #[test]
    fn it_calculate_power_consumption() {
        let data = super::parse_data("././data/test");
        assert_eq!(super::power_consumption(data),198);
    }
    #[test]
    fn it_solves1 () {
        let result = super::solve1("././data/sample1");
        assert_eq!(result,738234);
    }

    #[test]
    fn it_calcs_oxigen() {
        let binaries_set = super::parse_data("././data/test");
        let oxigen_rating = super::oxigen_rating(&binaries_set);
        assert_eq!(oxigen_rating,23);
    }

    #[test]
    fn it_cals_co2() {
        let binaries_set = super::parse_data("././data/test");
        let co2_rating = super::co2_rating(&binaries_set);
        assert_eq!(co2_rating,10);
    }
}
