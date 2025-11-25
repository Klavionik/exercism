const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len() as i32;
    
    if rows == 0 {
        return vec![]
    }
    
    let columns = garden[0].len() as i32;
    
    if columns == 0 {
        return vec![String::new()]
    }
    
    let mut result = vec![];

    for (y, row) in garden.iter().enumerate() {
        let mut string = String::new();

        for (x, cell) in row.as_bytes().iter().enumerate() {
            if *cell == b'*' {
                string.push('*');
                continue
            }

            let mut flowers = 0;
            
            for (ox, oy) in OFFSETS.iter() {
                let (next_x, next_y) = (x as i32 + ox, y as i32 + oy);
                
                if (0 <= next_x && next_x < columns) && (0 <= next_y && next_y < rows) {
                    let cell = garden[next_y as usize].as_bytes()[next_x as usize];
                    
                    if cell == b'*' {
                        flowers += 1
                    }
                }
            }
            
            if flowers == 0 {
                string.push(' ');
            } else {
                string.push_str(&flowers.to_string());
            }
        }
        
        result.push(string);
    }

    result
}