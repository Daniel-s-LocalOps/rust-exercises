fn sort_usernames<T: AsRef<str>>(users: &mut Vec<T>) {
    // users.sort_by(|a, b| a.as_ref().to_lowercase().cmp(&b.as_ref().to_lowercase()));
    users.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}

fn main() {}

#[test]
fn should_sort_list_strings_alphabetically_when_same_case() {
    let mut names = vec!["daniel", "kevin", "paul", "aron"];
    let expected = vec!["aron", "daniel", "kevin", "paul"];

    sort_usernames(&mut names);
    assert_eq!(names, expected);
}

#[test]
fn should_sort_list_strings_alphabetically_when_mixed_case() {
    let mut names = vec!["DANIEL", "KeVin", "paul", "Aron"];
    let expected = vec!["Aron", "DANIEL", "KeVin", "paul"];

    sort_usernames(&mut names);
    assert_eq!(names, expected);
}

#[test]
fn should_not_panic_when_list_strings_is_empty() {
    let mut names: Vec<&str> = vec![];
    let expected: Vec<&str> = vec![];

    sort_usernames(&mut names);
    assert_eq!(names, expected);
}
