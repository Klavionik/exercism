pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = vec![];
    let bytes_minefield = minefield.iter().map(|row| row.as_bytes()).collect::<Vec<_>>();

    for (x, row) in bytes_minefield.iter().enumerate() {
        let mut string = String::new();

        for (y, cell) in row.iter().enumerate() {
            if *cell == b'*' {
                string.push('*');
                continue
            }

            let mut mines = 0;

            let maybe_up_y = y.checked_sub(1);
            let right_x = x + 1;
            let down_y = y + 1;
            let maybe_left_x = x.checked_sub(1);
            
            // Right.
            mines += is_mine(right_x, y, &bytes_minefield);
            // Down.
            mines += is_mine(x, down_y, &bytes_minefield);
            // Down right.
            mines += is_mine(right_x, down_y, &bytes_minefield);
            
            if let Some(up_y) = maybe_up_y {
                // Up.
                mines += is_mine(x, up_y, &bytes_minefield);
                // Up right.
                mines += is_mine(right_x, up_y, &bytes_minefield);
                
                if let Some(left_x) = maybe_left_x {
                    // Up left.
                    mines += is_mine(left_x, up_y, &bytes_minefield);
                }
            }
            
            if let Some(left_x) = maybe_left_x {
                // Left.
                mines += is_mine(left_x, y, &bytes_minefield);
                // Down left.
                mines += is_mine(left_x, down_y, &bytes_minefield);
            }
            
            if mines == 0 {
                string.push(' ');
            } else {
                string.push_str(&format!("{mines}"));
            }
        }
        result.push(string);
    }

    result
}

fn is_mine(x: usize, y: usize, minefield: &Vec<&[u8]>) -> u8 {
    let cell = minefield.get(x).and_then(|row| row.get(y));
    
    match cell {
        Some(value) if *value == b'*' => 1,
        _ => 0
    }
}