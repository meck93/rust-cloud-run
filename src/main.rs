fn main() {
    let statement = statement();
    println!("{:?}", statement);
}

fn statement() -> &'static str {
    "hello world from rust!"
}

#[cfg(test)]
mod tests {
    use crate::statement;

    #[test]
    fn it_should_test_main() {
        let statement = statement();
        assert_eq!(statement, "hello world from rust!");
    }
}
