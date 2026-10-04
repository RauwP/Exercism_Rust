pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let rows = diagram.split('\n').collect::<Vec<&str>>();
    let children = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let base_idx = children.iter().position(|name| name.eq(&student)).unwrap() * 2;

    let mut res = Vec::new();

    let pos = [
        (0, base_idx),
        (0, base_idx + 1),
        (1, base_idx),
        (1, base_idx + 1),
    ];
    for (r, c) in pos {
        res.push(match rows[r].chars().nth(c).unwrap() {
            'V' => "violets",
            'G' => "grass",
            'R' => "radishes",
            'C' => "clover",
            _ => unreachable!(),
        })
    }
    res
}
