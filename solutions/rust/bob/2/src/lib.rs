pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_silence = message.is_empty();

    if is_silence {
        return "Fine. Be that way!"
    }

    let is_question = message.ends_with("?");

    let message = message.strip_suffix("?").unwrap_or(message);
    let is_yelling = message.chars().any(|x| x.is_alphabetic()) && message == message.to_uppercase();

    if is_question && is_yelling {
        return "Calm down, I know what I'm doing!"
    }

    if is_question && !is_yelling {
        return "Sure."
    }

    if !is_question && is_yelling {
        return "Whoa, chill out!"
    }

    "Whatever."
}