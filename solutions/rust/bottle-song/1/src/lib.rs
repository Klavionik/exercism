use std::collections::HashMap;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = vec![];

    let numbers = HashMap::from([
        (0, "no"),
        (1, "One"),
        (2, "Two"),
        (3, "Three"),
        (4, "Four"),
        (5, "Five"),
        (6, "Six"),
        (7, "Seven"),
        (8, "Eight"),
        (9, "Nine"),
        (10, "Ten"),
    ]);

    let mut current = start_bottles;
    let mut left = take_down;

    while left > 0 {
        let next = current - 1;

        let line_1 = format!(
            "{} green {} hanging on the wall,\n",
            numbers[&current],
            if current == 1 { "bottle" } else { "bottles" }
        );
        let line_3 = "And if one green bottle should accidentally fall,\n";
        let line_4 = format!(
            "There'll be {} green {} hanging on the wall.",
            numbers[&next].to_ascii_lowercase(),
            if next == 1 { "bottle" } else { "bottles" }
        );

        let verse = format!("{}{}{}{}{}", line_1, line_1, line_3, line_4, if left == 1 { "" } else { "\n" });
        verses.push(verse);

        current -= 1;
        left -= 1;
    }

    verses.join("\n")
}