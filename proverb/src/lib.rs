pub fn build_proverb(list: &[&str]) -> String {
    let mut res = <Vec<String>>::new();
    if list.len() != 0 {
        res = list
            .windows(2)
            .map(|dou| format!("For want of a {} the {} was lost.", dou[0], dou[1]))
            .collect::<Vec<String>>();
        res.push(format!("And all for the want of a {}.", list[0]));
    }
    res.join("\n")
}
