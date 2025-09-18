fn main() {}

fn unlique<T: Ord>(mut values: Vec<T>) -> Vec<T> {
    let mut previous: usize = 0;

    for current in 0..values.len() {
        if current == previous {
            continue;
        }

        if values[previous] == values[current] {
            values.remove(current);
        }

        if current == values.len() {
            previous += 1;
        }
    }

    values
}

#[test]
fn should_return_list_unchanged_when_no_duplicates_exist() {
    let list = vec![1, 2, 3];

    let expected = vec![1, 2, 3];
    assert_eq!(unlique(list), expected);
}

#[test]
fn should_return_unqiue_list_when_contains_duplicates() {
    let list = vec![3, 2, 3];

    let expected = vec![3, 2];
    assert_eq!(unlique(list), expected);
}

#[test]
fn should_return_empty_list_when_list_has_no_elements() {
    let list: Vec<i32> = vec![];
    let expected: Vec<i32> = vec![];

    assert_eq!(unlique(list), expected);
}
