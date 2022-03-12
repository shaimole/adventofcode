
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_locatates_data_file() {

        assert_eq!(2 + 2, 4);
    }
}
