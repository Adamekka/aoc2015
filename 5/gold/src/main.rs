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

    assert!(is_nice_str("qjhvhtzxzqqjkmpb"));
    assert!(is_nice_str("xxyxx"));
    assert!(contains_pair("uurcxstgmygtbstg"));
    assert!(!contains_repeating_letter("uurcxstgmygtbstg"));
    assert!(!contains_pair("ieodomkazucvgmuy"));
    assert!(contains_repeating_letter("ieodomkazucvgmuy"));

    assert_eq!(nice_strings, 55);
}

fn is_nice_str(str: &str) -> bool {
    contains_pair(str) && contains_repeating_letter(str)
}

fn contains_pair(str: &str) -> bool {
    let mut last_c1 = '\0';
    for (idx, c1) in str.chars().enumerate() {
        if last_c1 == '\0' {
            last_c1 = c1;
            continue;
        }

        let pair1 = [last_c1, c1];
        last_c1 = c1;
        let str2 = &str[idx + 1..];

        let mut last_c2 = '\0';
        for c2 in str2.chars() {
            if last_c2 == '\0' {
                last_c2 = c2;
                continue;
            }

            let pair2 = [last_c2, c2];
            last_c2 = c2;

            if pair1 == pair2 {
                return true;
            }
        }
    }

    false
}

fn contains_repeating_letter(str: &str) -> bool {
    for (idx, c) in str.chars().enumerate() {
        if c == str.chars().nth(idx + 2).unwrap_or('\0') {
            return true;
        }
    }

    false
}
