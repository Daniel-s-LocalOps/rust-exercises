fn main() {
    let values = vec![2.0, 3.0, 1.0, 4.0];

    match median(values) {
        Some(value) => println!("the result is: {}", value),
        None => println!("No result returned, possible empty array"),
    };
}

fn median_value(mut values: Vec<f32>) -> Option<f32> {
    if values.is_empty() {
        return None;
    }

    // This is not really possible since we are hardcoding the values
    // but this will guarantee the next line will never panic
    values.retain(|x| !x.is_nan());

    // unwrap will only panic if the vec of f32 contains a NaN value
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // this will always work because usize will round up so 2.5 will become 3
    let middle_index: usize = values.len() / 2;

    match values.len() % 2 {
        1 => Some(values[middle_index]),
        _ => Some((values[middle_index] + values[middle_index - 1]) / 2.0),
    }
}

// made this a little more generic so it can support other types
// it still converts it to a f64 which is needed in order to handle
// cases that the number of elements is even and the program needs to add then divide them
fn median<T: TryInto<f64>>(values: Vec<T>) -> Option<f64> {
    let mut values: Vec<f64> = values
        .into_iter()
        .filter_map(|x| match x.try_into() {
            Ok(f_value) if !f_value.is_nan() => Some(f_value),
            _ => None,
        })
        .collect();

    if values.is_empty() {
        return None;
    }

    // unwrap will only panic if the vec of f32 contains a NaN value
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // this will always work because usize will round up so 2.5 will become 3
    let middle_index: usize = values.len() / 2;

    Some(match values.len() % 2 {
        1 => values[middle_index],
        _ => (values[middle_index] + values[middle_index - 1]) / 2.0,
    })
}

#[test]
fn should_work_as_usual_when_list_contains_nan() {
    let input: Vec<f32> = vec![10.0, 20.0, 4.0, f32::NAN];

    let expected_result = Some(10.0);
    let actual_result = median(input);
    assert_eq!(actual_result, expected_result);
}

#[test]
fn should_return_some_when_type_is_i32() {
    let input: Vec<i32> = vec![10, 20, 4];

    let expected_result = Some(10.0);
    let actual_result = median(input);
    assert_eq!(actual_result, expected_result);
}

#[test]
fn should_return_none_when_list_is_empty() {
    let input: Vec<f32> = vec![];

    let expected_result = None;
    let actual_result = median(input);
    assert_eq!(actual_result, expected_result);
}

#[test]
fn should_return_some_when_list_contains_even_number_of_elements() {
    let input: Vec<f32> = vec![40.0, 20.0];

    let expected_result = Some(30.0);
    let actual_result = median(input);
    assert_eq!(actual_result, expected_result);
}

#[test]
fn should_return_some_when_list_contains_odd_number_of_elements() {
    let input: Vec<f32> = vec![10.0, 50.0, 30.0];

    let expected_result = Some(30.0);
    let actual_result = median(input);
    assert_eq!(actual_result, expected_result);
}
