#[must_use]
pub fn actions(n: u8) -> Vec<&'static str> {
    const ACTION_COUNT: usize = 4;
    let actions: [&str; ACTION_COUNT] = ["wink", "double blink", "close your eyes", "jump"];
    let secret_handshake = actions
        .into_iter()
        .enumerate()
        .filter_map(|(bit, action)| {
            let bitmask = 1 << bit;
            (n & bitmask != 0).then_some(action)
        });

    if n & (1 << ACTION_COUNT) == 0 {
        secret_handshake.collect()
    } else {
        secret_handshake.rev().collect()
    }
}
