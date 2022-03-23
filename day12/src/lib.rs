 pub fn solve1() -> u16 {
    1
 }


pub fn solve2() -> u16 {
    1
}
 

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_solve_first_test_case() {
        assert_eq!(super::solve1("././data/test"),10)
    }

    #[test]
    fn it_should_solve_second_test_case() {
        assert_eq!(super::solve1("././data/test2"),19)
    }

        #[test]
    fn it_should_solve_third_test_case() {
        assert_eq!(super::solve1("././data/test3"),226)
    }
}
