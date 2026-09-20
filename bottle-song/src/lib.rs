pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn number_word(n: u32) -> &'static str {
    match n {
        10 => "Ten",
        9 => "Nine",
        8 => "Eight",
        7 => "Seven",
        6 => "Six",
        5 => "Five",
        4 => "Four",
        3 => "Three",
        2 => "Two",
        1 => "One",
        0 => "no",
        _ => unreachable!(),
    }
}

fn bottles(n: u32) -> String {
    format!(
        "{} green bottle{}",
        number_word(n),
        if n == 1 { "" } else { "s" },
    )
}

fn verse(n: u32) -> String {
    let bottles_str = bottles(n);
    let bottles_str_minus_one = bottles(n - 1).to_lowercase();
    format!(
        "{bottles_str} hanging on the wall,\n{bottles_str} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {bottles_str_minus_one} hanging on the wall."
    )
}
