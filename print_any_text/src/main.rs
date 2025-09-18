fn main() {}

fn info<T: AsRef<str>>(text: &T) {
    let s: &str = text.as_ref();
    println!("{}", s);
}

#[test]
fn should_print_when_string_slice() {
    info(&"this is a test");
}

#[test]
fn should_print_when_string() {
    info(&String::from("this is a test"));
}
