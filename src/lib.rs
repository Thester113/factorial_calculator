#[no_mangle]
pub extern "C" fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(2);
        assert_eq!(result, 2);
    }
}
