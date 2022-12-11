use std::path::Path;

struct Monkey<T: ?Sized> {
    items: Vec<u32>,
    operation: Box<T>,
}

fn init_monkeys() -> Vec<Monkey<dyn Fn(usize) -> (usize, u32)>>{

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

fn parse<P>(filename : P ) -> Vec<Vec<u32>> 
where P: AsRef<Path>, {
  vec![vec![]] 
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
        assert_eq!(solve(),10605)
    }

    #[test]
    fn it_should_solve_sample_part2() {
        assert_eq!(solve2("./data/sample"),8)
    }
    #[test]
    // fn it_should_solve_part_1() {
    //      assert_eq!(solve("./data/input"),1789)
    // }

    #[test]
    fn it_should_solve_part_2() {
        assert_eq!(solve2("./data/input"),314820)
    }

}
