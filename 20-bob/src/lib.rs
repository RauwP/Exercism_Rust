pub fn reply(message: &str) -> &str {
    let quest = message.trim().ends_with('?');
    let letters_only: String = message.chars().filter(|&ch| ch.is_alphabetic()).collect();
    let all_caps = letters_only.chars().all(|ch| ch.is_uppercase()) && !letters_only.is_empty();
    let silence = message.chars().all(|ch| ch.is_whitespace());

    match (quest, all_caps, silence) {
        (true, false, false) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (true, true, false) => "Calm down, I know what I'm doing!",
        (_, _, true) => "Fine. Be that way!",
        (false, false, false) => "Whatever.",
    }
}
