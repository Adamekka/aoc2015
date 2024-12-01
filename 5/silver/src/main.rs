trait ContainsMultiple {
    fn contains_multiple(&self, pat: char) -> u8;
}

impl ContainsMultiple for str {
    fn contains_multiple(&self, pat: char) -> u8 {
        let mut res: u8 = 0;
        for c in self.chars() {
            if c == pat {
                res += 1;
            }
        }

        res
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    let lines = text.split('\n');

    let mut nice_strings: u16 = 0;

    for line in lines {
        if is_nice_str(line) {
            nice_strings += 1;
        }
    }

    println!("{}", nice_strings);

    assert!(is_nice_str("ugknbfddgicrmopn"));
    assert!(is_nice_str("aaa"));
    assert!(!is_nice_str("jchzalrnumimnmhp"));
    assert!(!is_nice_str("haegwjzuvuyypxyu"));
    assert!(!is_nice_str("dvszwmarrgswjxmb"));
}

fn is_nice_str(str: &str) -> bool {
    contains_vowels(str) && contains_letter_twice(str) && does_not_contain_strings(str)
}

fn contains_vowels(str: &str) -> bool {
    let mut vowel_count: u8 = 0;

    vowel_count += str.contains_multiple('a');
    vowel_count += str.contains_multiple('e');
    vowel_count += str.contains_multiple('i');
    vowel_count += str.contains_multiple('o');
    vowel_count += str.contains_multiple('u');

    vowel_count >= 3
}

fn contains_letter_twice(str: &str) -> bool {
    let mut last_char = '\0';

    for c in str.chars() {
        if c == last_char {
            return true;
        }

        last_char = c;
    }

    false
}

fn does_not_contain_strings(str: &str) -> bool {
    if str.contains("ab") {
        return false;
    }

    if str.contains("cd") {
        return false;
    }

    if str.contains("pq") {
        return false;
    }

    if str.contains("xy") {
        return false;
    }

    true
}
