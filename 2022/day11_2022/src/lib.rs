
struct Monkey<T: ?Sized> {
    items: Vec<Vec<u32>>,
    operation: Box<T>,
}

fn init_monkeys_sample() -> Vec<Monkey<dyn Fn(usize) -> (usize, u32)>>{

    let felix_ops = Box::new(
        |item: usize| {
            let new_worry = (item * 19) %(17*13*19*23); 
            if (new_worry%23 == 0) {
                return (new_worry,2)
            }
            (new_worry as usize, 3u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let felix = Monkey {
        items: vec![vec![79;4],vec![98;4]],
        operation: felix_ops
    };

    let lars_ops = Box::new(
        |item: usize| {
            let new_worry = (item + 6) % (17*13*19*23); 
            if new_worry%19 == 0 {
                return (new_worry,2)
            }
            (new_worry as usize, 0u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let lars = Monkey {
        items: vec![
            vec![54;4],vec![65;4],vec![75;4],vec![74;4]],
        operation: lars_ops,
    };

     let jens_ops = Box::new(
        |item: usize| {
            let new_worry = (item * item) %(17*13*19*23); 
            if new_worry%13 == 0 {
                return (new_worry,1)
            }
            (new_worry as usize, 3u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let jens = Monkey {
        items: vec![
             vec![79;4],vec![60;4],vec![97;4]
             ],
        operation: jens_ops,
    };

    let raik_ops = Box::new(
        |item: usize| {
            let new_worry = (item + 3 )% (17*13*19*23);
            if new_worry%17 == 0 {
                return (new_worry,0)
            }
            (new_worry as usize, 1u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let raik = Monkey {
        items: vec![
            vec![74;4]
            ],
        operation: raik_ops,
    };
    let apes =  vec![felix,lars,jens,raik];
    apes
   
}


fn init_monkeys() -> Vec<Monkey<dyn Fn(usize) -> (usize, u32)>>{


    let felix_ops = Box::new(
        |item: usize| {
            let modulo = 2;
            let new_worry = (item* 5) % 9699690; 
            if (new_worry%modulo == 0) {
                return (new_worry,4)
            }
            (new_worry as usize, 3u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let lars_ops = Box::new(
        |item: usize| {
            let modulo = 7;
            let success = 5;
            let fail = 6;
            let new_worry = (item + 7) % 9699690; 
            if new_worry%modulo == 0 {
                return (new_worry,success)
            }
            (new_worry as usize, fail as u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

     let jens_ops = Box::new(
        |item: usize| {
            let modulo = 3;
            let success = 7;
            let fail = 0;
            let new_worry = (item +5) % 9699690;; 
            if new_worry%modulo == 0 {
                return (new_worry,success)
            }
            (new_worry as usize, fail as u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let raik_ops = Box::new(
        |item: usize| {
            let modulo = 17;
            let success = 1;
            let fail = 5;
            let new_worry = (item + 8) % 9699690;
            if new_worry%modulo == 0 {
                return (new_worry,success)
            }
            (new_worry as usize, fail as u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let felix = Monkey {
        items: vec![vec![80;8]],
        operation: felix_ops
    };

    let lars = Monkey {
        items: vec![
            vec![75;8],vec![83;8],vec![74;8]],
        operation: lars_ops,
    };
    let jens = Monkey {
        items: vec![
             vec![86;8],vec![67;8],vec![61;8],vec![96;8],vec![52;8],vec![63;8], vec![73;8]
             ],
        operation: jens_ops,
    };
    let raik = Monkey {
        items: vec![
            vec![85;8],vec![83;8],vec![55;8],vec![85;8],vec![57;8],vec![70;8],vec![85;8],vec![52;8]
            ],
        operation: raik_ops,
    };
   

    let tom_ops = Box::new(
        |item: usize| {
            let modulo = 11;
            let success = 3;
            let fail = 1;
            let new_worry = (item + 4) % 9699690;
            if new_worry%modulo == 0 {
                return (new_worry,success)
            }
            (new_worry as usize, fail as u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let tom = Monkey {
        items: vec![vec![67;8],vec![75;8],vec![91;8],vec![72;8],vec![89;8]],
        operation: tom_ops,
    };

    let leo_ops = Box::new(
        |item: usize| {
            let modulo = 19;
            let success = 6;
            let fail = 2;
            let new_worry = (item* 2) % 9699690;
            if new_worry%modulo == 0 {
                return (new_worry,success)
            }
            (new_worry as usize, fail as u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let leo = Monkey {
        items:vec![vec![66;8],vec![64;8],vec![68;8],vec![92;8],vec![68;8],vec![77;8]],
        operation: leo_ops,
    };

    let alfred_ops = Box::new(
        |item: usize| {
            let modulo = 5;
            let success = 2;
            let fail = 7;
            let new_worry = (item * item) % 9699690; 
            if new_worry%modulo == 0 {
                return (new_worry,success)
            }
            (new_worry as usize, fail as u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let alfred = Monkey {
        items:vec![vec![97;8],vec![94;8],vec![79;8],vec![88;8]],
        operation: alfred_ops,
    };

    let sepp_ops = Box::new(
        |item: usize| {
            let modulo = 13;
            let success = 4;
            let fail = 0;
            let new_worry = (item + 6) % 9699690; 
            if new_worry%modulo == 0 {
                return (new_worry,success)
            }
            (new_worry as usize, fail as u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let sepp = Monkey {
        items:vec![vec![77;8],vec![85;8]],
        operation: sepp_ops,
    };



    let apes =  vec![felix,lars,jens,raik,tom,leo,alfred,sepp];
    apes
   
}



pub fn solve_sample(steps: usize) -> u32  {
    let mut apes = init_monkeys_sample();
    let mut operations: Vec<u32> = vec![0;apes.len()];
    for round in 0..steps {
        for n in 0..apes.len() {
            for _ in 0..apes[n].items.len() {
                operations[n] = operations[n] +1;
                let operation = &apes[n].operation;
                let ape_result: (usize,u32) = operation(apes[n].items[0][n] as usize);
                let mut new_items = apes[n].items[0].clone();
                new_items = vec![ape_result.0 as u32;4];
                apes[ape_result.1 as usize].items.push(new_items);
                apes[n].items.remove(0);
            }
        }
    }
    println!("{:?}",operations);
    operations.sort();
    let max = operations.pop().unwrap();
    return max * operations.pop().unwrap();
}

pub fn solve(steps: usize) -> u128  {
    let mut apes = init_monkeys();
    let mut operations: Vec<u32> = vec![0;apes.len()];
    for round in 0..steps {
        for n in 0..apes.len() {
            for _ in 0..apes[n].items.len() {
                operations[n] = operations[n] +1;
                let operation = &apes[n].operation;
                let ape_result: (usize,u32) = operation(apes[n].items[0][n] as usize);
                let mut new_items = apes[n].items[0].clone();
                new_items[n] = ape_result.0 as u32;
                new_items = vec![ape_result.0 as u32;8];
                apes[ape_result.1 as usize].items.push(new_items);
                apes[n].items.remove(0);
            }
        }
    }
    operations.sort();
    let max = operations.pop().unwrap();
    return max as u128 * operations.pop().unwrap() as u128;
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve_sample(20),10605)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve_sample(10000),2713310158)
    }
    #[test]
    fn it_should_solve_part_1() {
         assert_eq!(solve(20),100345)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve(10000),314820)
    }

}
