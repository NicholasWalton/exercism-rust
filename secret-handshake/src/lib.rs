#[must_use]
pub fn actions(n: u8) -> Vec<&'static str> {
    let mut secret_handshake = vec![];
    if n & 0b1 != 0 {
        secret_handshake.push("wink");
    }
    if n & 0b10 != 0 {
        secret_handshake.push("double blink");
    }
    if n & 0b100 != 0 {
        secret_handshake.push("close your eyes");
    }
    if n & 0b1000 != 0 {
        secret_handshake.push("jump");
    }
    if n & 0b1_0000 != 0 {
        secret_handshake.reverse();
    }
    secret_handshake
}
