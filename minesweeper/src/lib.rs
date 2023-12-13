pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let result: Vec<String> = minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let annotated_row: String = row
                .chars()
                .enumerate()
                .map(|(j, square)| {
                    if square == '*' {
                        '*'.to_string()
                    } else {
                        let count = count_mines(i, j, minefield);
                        if count > 0 {
                            char::from_digit(count, 10).unwrap().to_string()
                        } else {
                            ' '.to_string()
                        }
                    }
                })
                .collect();
            annotated_row
        })
        .collect();
    result
}

fn count_mines(i: usize, j: usize, minefield: &[&str]) -> u32 {
    let mut count = 0;
    for x in i.saturating_sub(1)..=(i + 1).min(minefield.len() - 1) {
        for y in j.saturating_sub(1)..=(j + 1).min(minefield[0].len() - 1) {
            if (x != i || y != j) && minefield[x].chars().nth(y).unwrap() == '*' {
                count += 1;
            }
        }
    }
    count
}
