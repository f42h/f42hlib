mod macros;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_test_os() {
        dbg!(tern!(1 == 1, true, false));
    }

    #[test] 
    fn macro_test_tern() {
        dbg!(get_os!());
    }
}
