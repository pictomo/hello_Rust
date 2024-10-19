fn main() {
    let result: Result<&str, &str> = checker("hello");
    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("Error: {}", e),
    }
}

fn checker(s: &str) -> Result<&str, &str> {
    if s == "hello" {
        return Ok(s);
    } else {
        return Err(r#"Not "hello""#);
    }
}
