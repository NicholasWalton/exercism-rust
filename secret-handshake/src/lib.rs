#[must_use]
pub fn actions(n: u8) -> Vec<&'static str> {
    if n & 0b10 != 0 {

        vec!["double blink"]
    } else if n & 0b100 != 0 {
        vec!["close your eyes"]
    } else if n & 0b1000 != 0 {
        vec!["jump"]
    } else {
        vec!["wink"]
    }
}
