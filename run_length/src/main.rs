use std::arch::is_aarch64_feature_detected;

fn encode(encode_string: &str) -> String {
    let mut wordmap: Vec<String> = Vec::new();

    // convert the string into a list of chars
    // take the first char and c && index != encode_string.len()unt duplicates up to a new char is found
    // create a new(index, ) str)ing with the number found (1) if only one char found
    // once char not found, use that as the new chat to check and repeat

    let mut current: Option<char> = None;
    let mut current_count = 1;

    for (index, achar) in encode_string.chars().enumerate() {
        if current.is_none() {
            current = Some(achar);
            println!("current char is {} inside none", achar);
            continue;
        }

        if current.unwrap() == achar {
            current_count += 1;

            if index != encode_string.chars().count() {
                continue;
            }
        }

        if index == encode_string.chars().count() {
            current = Some(achar);
        }

        println!(
            "char count {} index {}",
            encode_string.chars().count(),
            index
        );

        let current_word = format!("{}{}", current_count, current.unwrap());
        wordmap.push(current_word.to_string());

        current = Some(achar);
        current_count = 1;
    }

    println!("{:?}", wordmap);

    wordmap.join("")
}

fn encode_better(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();

    let mut encoded = String::new();

    let mut current: usize = 0;
    let mut next: usize = current + 1;

    while current < chars.len() {
        if chars[current] == chars[next] {
            next = if next + 1 < chars.len() {
                next + 1
            } else {
                chars.len()
            };

            if next != chars.len() {
                continue;
            }
        }

        let slice: String = chars[current..next].iter().collect();
        let format = format!("{}{}", slice.len(), slice);

        println!("{}", format);
        encoded.push_str(&format);

        // do at the very end
        current = next;
        next = if next + 1 < chars.len() {
            next + 1
        } else {
            chars.len()
        }
    }

    encoded
}

fn main() {
    println!("{}", encode_better("Daniel"));
    //println!("{}", encode("AAAAAaAA"));
}
