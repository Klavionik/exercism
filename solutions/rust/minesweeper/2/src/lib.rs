const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len() as i32;
    
    if rows == 0 {
        return vec![]
    }
    
    let columns = minefield[0].len() as i32;
    
    if columns == 0 {
        return vec![String::new()]
    }
    
    let mut result = vec![];
    let bytes_minefield = minefield.iter().map(|row| row.as_bytes()).collect::<Vec<_>>();

    for (y, row) in bytes_minefield.iter().enumerate() {
        let mut string = String::new();

        for (x, cell) in row.iter().enumerate() {
            if *cell == b'*' {
                string.push('*');
                continue
            }

            let mut mines = 0;
            
            for (ox, oy) in OFFSETS.iter() {
                let (next_x, next_y) = (x as i32 + ox, y as i32 + oy);
                
                if (0 <= next_x && next_x < columns) && (0 <= next_y && next_y < rows) {
                    let cell = bytes_minefield[next_y as usize][next_x as usize];
                    
                    if cell == b'*' {
                        mines += 1
                    }
                }
            }
            
            if mines == 0 {
                string.push(' ');
            } else {
                string.push_str(&mines.to_string());
            }
        }
        
        result.push(string);
    }

    result
}