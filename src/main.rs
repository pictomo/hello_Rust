use regex::Regex;

fn main() {
    let result: Result<&str, &str> = checker("-1.992");
    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("Error: {}", e),
    }
}

fn checker(s: &str) -> Result<&str, &str> {
    let num_re: Regex = Regex::new(r"^-?[0-9]+(.[0-9]*)?$").unwrap();

    if num_re.is_match(s) {
        return Ok(s);
    } else {
        return Err(r#"Not a number"#);
    }
}
