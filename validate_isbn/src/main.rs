use std::str::FromStr;

// check the string are all digits and the length is 13 (is that minus the check digit?)

#[derive(Debug)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum ValidationError {
    InputTooLong,
    InputTooShort,
    InvalidInput(String),
    FailedChecksum,
}

impl FromStr for Isbn {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ValidationError::InvalidInput(
                "The string is empty".to_string(),
            ));
        }

        if !s.contains('-') {
            return Err(ValidationError::InvalidInput(
                "Does not contain - seprators".to_string(),
            ));
        }

        let vec = s
            .chars()
            .filter(|x| *x != '-')
            .filter_map(|x| x.to_digit(10))
            .map(|x| x as u8)
            .collect::<Vec<u8>>();

        if vec.len() > 13 {
            return Err(ValidationError::InputTooLong);
        }

        if vec.len() < 13 {
            return Err(ValidationError::InputTooShort);
        }

        let check_digit = vec.last().unwrap();

        if checksum(&vec[..vec.len() - 1]) != *check_digit {
            return Err(ValidationError::FailedChecksum);
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits: vec,
        })
    }
}

fn checksum(list: &[u8]) -> u8 {
    let weights = vec![1, 3];

    let sum: u8 = list
        .iter()
        .enumerate()
        .map(|(index, value)| value * weights[index % 2])
        .sum();

    let result = 10 - (sum % 10);

    match result {
        10 => 0,
        _ => result,
    }
}

fn main() {
    let isbn: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("{:?}", isbn);
}
