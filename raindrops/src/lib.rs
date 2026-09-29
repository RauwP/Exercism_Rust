pub fn raindrops(n: u32) -> String {
    let sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let res: String = sounds
        .iter()
        .filter(|(num, _)| n % num == 0)
        .map(|&(_, snd)| snd)
        .collect();
    if res.is_empty() {
        return n.to_string();
    }
    res
}
