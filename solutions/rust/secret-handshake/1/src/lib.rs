const WINK: (u8, &str) = (1, "wink");
const DOUBLE_BLINK: (u8, &str) = (1 << 1, "double blink");
const CLOSE_EYES: (u8, &str) = (1 << 2, "close your eyes");
const JUMP: (u8, &str) = (1 << 3, "jump");
const REVERSE: u8 = 1 << 4;

const ACTIONS: [(u8, &str); 4] = [WINK, DOUBLE_BLINK, CLOSE_EYES, JUMP];

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut output = vec![];

    for (code, action) in ACTIONS {
        if n & code != 0 {
            output.push(action)
        }
    }

    if n & REVERSE != 0 {
        output.reverse()
    }

    output
}