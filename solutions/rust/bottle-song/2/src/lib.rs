pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = vec![];

    for (left, current) in (0..take_down).rev().zip((0..=start_bottles).rev()) {
        let next = current - 1;
        let last = left == 0;

        let line_1 = format!(
            "{} green {} hanging on the wall,\n",
            get_string_for_num(current),
            if current == 1 { "bottle" } else { "bottles" }
        );
        let line_3 = "And if one green bottle should accidentally fall,\n";
        let line_4 = format!(
            "There'll be {} green {} hanging on the wall.",
            get_string_for_num(next).to_lowercase(),
            if next == 1 { "bottle" } else { "bottles" }
        );

        let verse = format!(
            "{}{}{}{}{}",
            line_1,
            line_1,
            line_3,
            line_4,
            if last { "" } else { "\n" }
        );
        verses.push(verse);
    }

    verses.join("\n")
}

fn get_string_for_num(num: u32) -> &'static str {
    match num {
        0 => "no",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => unreachable!(),
    }
}