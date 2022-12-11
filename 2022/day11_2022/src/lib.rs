use std::path::Path;

struct Monkey<T: ?Sized> {
    items: Vec<u32>,
    operation: Box<T>,
}

fn init_monkeys_sample() -> Vec<Monkey<dyn Fn(usize) -> (usize, u32)>>{

    let felix_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 * 19.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%23 == 0) {
                return (calm_worry,2)
            }
            (calm_worry as usize, 3u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let felix = Monkey {
        items: vec![79,98],
        operation: felix_ops
    };

    let lars_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 + 6.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%19 == 0) {
                return (calm_worry,2)
            }
            (calm_worry as usize, 0u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

     let jens_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 * item as f32;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%13 == 0) {
                return (calm_worry,1)
            }
            (calm_worry as usize, 3u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let raik_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 + 3.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
              println!("raik new worry {:?}, calm worry {:?}",new_worry, calm_worry);
            if (calm_worry%17 == 0) {
                return (calm_worry,0)
            }
            (calm_worry as usize, 1u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;


    let lars = Monkey {
        items: vec![54,65,75,74],
        operation: lars_ops,
    };
    let jens = Monkey {
        items: vec![79,60,97],
        operation: jens_ops,
    };
    let raik = Monkey {
        items: vec![74],
        operation: raik_ops,
    };
    let apes =  vec![felix,lars,jens,raik];
    apes
   
}

fn init_monkeys() -> Vec<Monkey<dyn Fn(usize) -> (usize, u32)>>{

    let felix_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 * 5.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%2 == 0) {
                return (calm_worry,4)
            }
            (calm_worry as usize, 3u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let felix = Monkey {
        items: vec![80],
        operation: felix_ops
    };



    let lars_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 + 7.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%7 == 0) {
                return (calm_worry,5)
            }
            (calm_worry as usize, 6u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let lars = Monkey {
        items: vec![75,83,74],
        operation: lars_ops,
    };



    let jens_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 + 5.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%3 == 0) {
                return (calm_worry,7)
            }
            (calm_worry as usize, 0u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let jens = Monkey {
        items: vec![86, 67, 61, 96, 52, 63, 73],
        operation: jens_ops,
    };



    let raik_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 + 8.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%17 == 0) {
                return (calm_worry,1)
            }
            (calm_worry as usize, 5u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let raik = Monkey {
        items: vec![85, 83, 55, 85, 57, 70, 85, 52],
        operation: raik_ops,
    };


    let tom_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 + 4.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%11 == 0) {
                return (calm_worry,3)
            }
            (calm_worry as usize, 1u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let tom = Monkey {
        items: vec![67, 75, 91, 72, 89],
        operation: tom_ops,
    };

    let leo_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 * 2.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%19 == 0) {
                return (calm_worry,6)
            }
            (calm_worry as usize, 2u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let leo = Monkey {
        items: vec![66, 64, 68, 92, 68, 77],
        operation: leo_ops,
    };

    let alfred_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 * item as f32;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%5 == 0) {
                return (calm_worry,2)
            }
            (calm_worry as usize, 7u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let alfred = Monkey {
        items: vec![97, 94, 79, 88],
        operation: alfred_ops,
    };

    let sepp_ops = Box::new(
        |item: usize| {
            let new_worry:f32 = item as f32 + 6.0;
            let calm_worry = (new_worry as f32 / 3.0) as usize;
            if (calm_worry%13== 0) {
                return (calm_worry,4)
            }
            (calm_worry as usize, 0u32)
        }
    ) as Box<dyn Fn(usize) -> (usize, u32)>;

    let sepp = Monkey {
        items: vec![77, 85],
        operation: sepp_ops,
    };




    let apes =  vec![felix,lars,jens,raik,tom,leo,alfred,sepp];
    apes
   
}



pub fn solve_sample() -> u32  {
    let mut apes = init_monkeys_sample();
    let mut operations: Vec<u32> = vec![0;apes.len()];
    for round in 0..20 {
        for n in 0..apes.len() {
            for _ in 0..apes[n].items.len() {
                operations[n] = operations[n] +1;
                let operation = &apes[n].operation;
                let ape_result: (usize,u32) = operation(apes[n].items[0] as usize);
                println!("{:?}", ape_result);
                apes[ape_result.1 as usize].items.push(ape_result.0 as u32);
                apes[n].items.remove(0);
            }
        }
        for i in 0..apes.len() {
            println!("round {:?}, {:?}",round, apes[i].items);
        }
    }
    operations.sort();
    let max = operations.pop().unwrap();
    return max * operations.pop().unwrap();
}

pub fn solve() -> u32  {
    let mut apes = init_monkeys();
    let mut operations: Vec<u32> = vec![0;apes.len()];
    for round in 0..20 {
        for n in 0..apes.len() {
            for _ in 0..apes[n].items.len() {
                operations[n] = operations[n] +1;
                let operation = &apes[n].operation;
                let ape_result: (usize,u32) = operation(apes[n].items[0] as usize);
                println!("{:?}", ape_result);
                apes[ape_result.1 as usize].items.push(ape_result.0 as u32);
                apes[n].items.remove(0);
            }
        }
        for i in 0..apes.len() {
            println!("round {:?}, {:?}",round, apes[i].items);
        }
    }
    operations.sort();
    let max = operations.pop().unwrap();
    return max * operations.pop().unwrap();
}

pub fn solve2<P>(filename : P) -> u32 
where P: AsRef<Path>, {
 0
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_solve_sample() {
        assert_eq!(solve_sample(),10605)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"),8)
    }
    #[test]
    fn it_should_solve_part_1() {
         assert_eq!(solve(),1789)
    }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"),314820)
    }

}
