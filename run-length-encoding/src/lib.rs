pub fn encode(source: &str) -> String {
    let mut dest: Vec<String> = vec![];
    let mut count: i64 = 0;
    let Some(mut current) = source.chars().next() else { return String::default() };
    for char in source.chars() {
        if char == current {
            count += 1;
        } else {
            push_count(&mut dest, count, current);
            current = char;
            count = 1;
        }
    }
    push_count(&mut dest, count, current);
    dest.into_iter().collect()
}

fn push_count(dest: &mut Vec<String>, count: i64, current: char) {
    if (count > 1) {
        dest.push(count.to_string());
    }
    dest.push(current.to_string());
}

pub fn decode(source: &str) -> String {
    let mut digits: Vec<char> = vec![];
    let mut dest: Vec<char> = vec![];
    for char in source.chars() {
        match char {
            '0'..='9' => {
                digits.push(char);
            }
            letter => {
                let count: String = digits.into_iter().collect();
                let count: i64 = count.parse().unwrap_or(1);
                for _ in 0..count {
                    dest.push(letter);
                }
                digits = vec![];
            }
        }
    }
    dest.into_iter().collect()
}
