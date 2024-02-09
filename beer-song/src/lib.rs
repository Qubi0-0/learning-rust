pub fn verse(n: u32) -> String {
    let inject_0 = "1 bottle";
    let inject_1 = "1 bottle";
    let inject_2 = "no more bottles";
    if n == 0 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else if n == 1 {
        format!(
            "{}{}{}{}{}{}",
            inject_0, // 0
            " of beer on the wall, ",
            inject_1, // 2
            " of beer.\nTake it down and pass it around, ",
            inject_2, // 4
            " of beer on the wall.\n"
        )
    } else {
        let inject_0_vec: Vec<_> = inject_0
            .split(" ")
            .map(|word| {
                if word.len() < 3 {
                    n.to_string()
                } else {
                    "bottles".to_string()
                }
            })
            .collect();
        let inject_1_vec: Vec<_> = inject_0
            .split(" ")
            .map(|word| {
                if word.len() < 3 {
                    n.to_string()
                } else {
                    "bottles".to_string()
                }
            })
            .collect();
        let inject_2_vec: Vec<_> = inject_0
            .split(" ")
            .map(|word| {
                if word.len() < 3 {
                    (n - 1).to_string()
                } else {
                    if (n - 1) > 1 {
                        "bottles".to_string()
                    } else {
                        "bottle".to_string()
                    }
                }
            })
            .collect();
        let inject_0 = inject_0_vec.join(" ");
        let inject_1 = inject_1_vec.join(" ");
        let inject_2 = inject_2_vec.join(" ");

        format!(
            "{}{}{}{}{}{}",
            inject_0, // 0
            " of beer on the wall, ",
            inject_1, // 2
            " of beer.\nTake one down and pass it around, ",
            inject_2, // 4
            " of beer on the wall.\n"
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();

    for verse_num in (end..=start).rev() {
        if !song.is_empty() {
            song.push_str("\n");
        }
        song.push_str(&verse(verse_num));
    }
    song
}
