pub fn encode(source: &str) -> String {
    // todo!("Return the run-length encoding of {source}.");
    source.to_string()
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
