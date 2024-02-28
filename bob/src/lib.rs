pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    let has_uppercase = msg.chars().any(|c| c.is_alphabetic() && c.is_uppercase());
    let is_yelling = has_uppercase
        && msg
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());

    if msg.chars().last() == Some('?') {
        if is_yelling {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if msg.is_empty() {
        "Fine. Be that way!"
    } else if is_yelling {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
