pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(x: usize) -> usize {
    x + 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn basic_add_two() {
        assert_eq!(add_two(3), 5);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }


}
