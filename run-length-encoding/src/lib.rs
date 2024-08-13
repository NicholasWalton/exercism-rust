#[must_use]
pub fn encode(source: &str) -> String {
    let mut dest: Vec<String> = vec![];
    let mut count: usize = 0;
    let Some(mut last_char) = source.chars().next() else {
        return String::default();
    };
    for current_char in source.chars() {
        if current_char == last_char {
            count += 1;
        } else {
            encode_char(&mut dest, count, last_char);
            last_char = current_char;
            count = 1;
        }
    }
    encode_char(&mut dest, count, last_char);
    dest.into_iter().collect()
}

fn encode_char(dest: &mut Vec<String>, count: usize, char: char) {
    if count > 1 {
        dest.push(count.to_string());
    }
    dest.push(char.to_string());
}

#[must_use]
pub fn decode(source: &str) -> String {
    let counts = source
        .split(|chr: char| chr.is_alphabetic() || chr.is_whitespace())
        .map(|num| num.parse::<usize>().unwrap_or(1));

    let letters = source.matches(|chr: char| chr.is_alphabetic() || chr.is_whitespace());

    let parsed = counts.zip(letters);

    parsed.map(|(count, letter)| letter.repeat(count)).collect()
}
