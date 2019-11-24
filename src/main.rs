use std::io;

fn is_vowel(c:char) -> Option<bool> {
    if let Some(lower) = c.to_lowercase().next() {
        return Some(lower == 'a' || lower == 'e' || lower == 'i' || lower == 'o' || lower == 'u');
    } else {
        return None;
    }
}

fn main() {
    loop {
        println!("Please input a word:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        let mut input = String::from(input.trim());

        let first = match input.chars().next() {
            None => {
                println!("Please input a valid word");
                continue;
            },
            Some(character) => character,
        };

        let vowel = match is_vowel(first) {
            None => {
                continue;
            },
            Some(result) => result,
        };

        if vowel {
            input.push_str("hay");
            println!("The first character is a vowel, output is {}", input);
            continue;
        }
        input.remove(0);
        input.push(first);
        input.push_str("ay");
        println!("The first character is a consonant, output is {}", input);
    }
}
