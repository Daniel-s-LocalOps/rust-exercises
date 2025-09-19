type Message = Vec<Letter>;
type Letter = Vec<Pulse>;

enum Pulse {
    Short,
    Long,
}

trait MorseCode {
    fn to_morse_code(&self) -> Message;
    fn display_morse(&self) -> String;
}

impl MorseCode for str {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;

        let mut message: Message = Vec::new();

        for char in self.chars() {
            let letter = match char.to_ascii_lowercase() {
                'a' => vec![Long, Short],
                'b' => vec![Long, Short, Short, Short],
                'c' => vec![Long, Short, Long, Short],
                _ => continue,
            };

            message.push(letter);
        }

        message
    }

    fn display_morse(&self) -> String {
        use Pulse::*;

        let mut morse_codes = Vec::new();

        self.to_morse_code().iter().for_each(|letter| {
            letter.iter().for_each(|pulse| {
                match pulse {
                    Short => morse_codes.push('.'),
                    Long => morse_codes.push('_'),
                };
            });

            morse_codes.push(' ');
        });

        morse_codes.iter().collect()
    }
}

fn main() {
    let string_message = "abc ccba";

    println!("{}", string_message.display_morse());
}
