// ************************
// * TEST TEST  TEST TEST *
// ************************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        assert!(false)
    }
}
