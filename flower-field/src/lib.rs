pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    ' ' => {
                        let sum = count_neighbors(garden, r, c);
                        match sum {
                            0 => ' ',
                            n => char::from_digit(n, 10).unwrap(),
                        }
                    }
                    _ => '*',
                })
                .collect()
        })
        .collect()
}

fn count_neighbors(garden: &[&str], r: usize, c: usize) -> u32 {
    let coords = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    coords
        .iter()
        .copied()
        .filter(|&(dr, dc)| Some(true) == is_flower(garden, r, c, dr, dc))
        .count() as u32
}

fn is_flower(garden: &[&str], r: usize, c: usize, dr: i32, dc: i32) -> Option<bool> {
    let rn = r.checked_add_signed(dr as isize)?;
    let cn = c.checked_add_signed(dc as isize)?;
    let row = garden.get(rn)?;
    let cell = row.as_bytes().get(cn)?;
    Some(*cell == b'*')
}
