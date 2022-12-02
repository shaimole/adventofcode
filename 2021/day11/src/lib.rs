
pub fn solve2() -> u16 {
   let mut squids = get_data();
   let mut step = 0;
   while !synchronized(&squids) {
      squids = run_step(squids).squids;
      step = step + 1;
   }
   return step;
}

pub fn solve1(steps: u16) -> u16 {
   let mut squids = get_data_sample();
   let mut total_flashes = 0;
   for _n in 0..steps {
      let step_result = run_step(squids);
      total_flashes += step_result.flashes;
      squids = step_result.squids;
   }
   return total_flashes;
}

struct StepResult {
    squids: [[u8;10];10],
    flashes: u16
}

fn synchronized(squids: &[[u8;10];10]) -> bool {
    const SQUID_GRID_SIZE: usize = 10;
    for i in 0..SQUID_GRID_SIZE {
        for j in 0..SQUID_GRID_SIZE {
            if squids[i][j] != 0 {
                return false;
            }
        }
    }
    return true;
}

fn run_step(mut squids: [[u8;10];10]) -> StepResult {
    let mut flashes = 0;
    squids = increase_energy_level(squids);
    while squid_is_discharging(&squids) {
        let flash_result = flash(squids);
        squids = flash_result.squids;
        flashes += flash_result.flashes;
    }
    return StepResult {
        squids: squids,
        flashes: flashes
    }
}


fn increase_energy_level(mut squids: [[u8;10];10]) -> [[u8;10];10] {
    for i in 0..squids.len() {
        for j in 0..squids[i].len() {
            squids[i][j] = squids[i][j] + 1;
        }
    }
    return squids;
}

fn squid_is_discharging(squids: &[[u8;10];10]) -> bool {
     for i in 0..squids.len() {
        for j in 0..squids[i].len() {
            if squids[i][j] >= 10 {
                return true;
            }
            
        }
    }
    return false;
}

fn flash(mut squids: [[u8;10];10]) -> StepResult {
    let mut flashes = 0;
    for i in 0..squids.len() {
        for j in 0..squids[i].len() {
            if squids[i][j] >= 10 {
                squids[i][j] = 0;
                flashes = flashes + 1;
                squids = increase_adjecent(squids,i,j);
            }
        }
    }
    return StepResult {
        squids: squids,
        flashes: flashes
    }
}

fn increase_adjecent(mut squids: [[u8;10];10], i :usize, j: usize) -> [[u8;10];10] {
    if i > 0 {
        if squids[i-1][j] != 0 {
            squids[i-1][j] = squids[i-1][j] + 1;
        }
    }
    if i < 9 {
        if squids[i+1][j] != 0 {
            squids[i+1][j] = squids[i+1][j] + 1;
        }
    }
    if j > 0 {
        if squids[i][j-1] != 0{
            squids[i][j-1] = squids[i][j-1] + 1;            
        }
       
    }
    if j < 9 {
        if squids[i][j+1] != 0{
            squids[i][j+1] = squids[i][j+1] + 1;
        }
        
    }

    if j < 9  && i < 9{
        if squids[i+1][j+1] != 0{
             squids[i+1][j+1] = squids[i+1][j+1] + 1;            
        }
    }

    if j > 0  && i > 0 {
        if squids[i-1][j-1] != 0{
            squids[i-1][j-1] = squids[i-1][j-1] + 1;   
        }
        
    }
    if j > 0  && i < 9 {
        if squids[i+1][j-1] != 0{
            squids[i+1][j-1] = squids[i+1][j-1] + 1;   
        }
        
    }
     if j < 9  && i > 0 {
        if squids[i-1][j+1] != 0{ 
              squids[i-1][j+1] = squids[i-1][j+1] + 1;   
        }
    }
    squids
}


fn get_data_sample() -> [[u8;10];10] {
    return [
            [5,4,8,3,1,4,3,2,2,3],
            [2,7,4,5,8,5,4,7,1,1],
            [5,2,6,4,5,5,6,1,7,3],
            [6,1,4,1,3,3,6,1,4,6],
            [6,3,5,7,3,8,5,4,7,8],
            [4,1,6,7,5,2,4,6,4,5],
            [2,1,7,6,8,4,1,7,2,1],
            [6,8,8,2,8,8,1,1,3,4],
            [4,8,4,6,8,4,8,5,5,4],
            [5,2,8,3,7,5,1,5,2,6]
    ];
}

fn get_data() -> [[u8;10];10] {
    return [
            [4,7,6,4,7,4,5,7,8,4],
            [4,6,4,3,4,5,7,1,7,6],
            [8,3,2,2,6,2,8,4,7,7],
            [7,6,1,7,1,5,2,5,4,6],
            [6,1,3,7,5,1,8,1,6,5],
            [1,5,5,6,7,2,3,1,7,6],
            [2,1,8,7,8,6,1,8,8,6],
            [2,5,5,3,4,2,2,6,2,5],
            [4,8,1,7,5,8,4,6,3,8],
            [3,7,5,4,2,8,5,6,6,2],
    ];
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_solve_testdata_for_part_1() {
       assert_eq!(super::solve1(10),204);
    }
    #[test]
    fn it_should_solve_testdata_for_part_2() {
       assert_eq!(super::solve2(),517);
    }
}
 
