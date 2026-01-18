fn build_grid(notes: &String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in notes.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    grid
}

fn check_neighbors(row_index: usize, col_index: usize, value: char, grid: &Vec<Vec<char>>) -> bool {
    let mut can_mine = true;
    let row_len = grid.len() as isize;
    let col_len = grid[0].len() as isize;
    let neighbors = [
        (-1, 0),
        (0, -1), (0, 1),
        (1, 0),
    ];
    for (dr, dc) in neighbors.iter() {
        let new_row = row_index as isize + dr;
        let new_col = col_index as isize + dc;
        if new_row < 0 || new_row >= row_len || new_col < 0 || new_col >= col_len {
            continue;
        }
        if grid[new_row as usize][new_col as usize] != value || value == '.' {
            can_mine = false;
        }
    }
    can_mine
}

pub fn mining_1(notes: String) -> i32 {
    let mut grid = build_grid(&notes);
    let mut changes: Vec<(usize, usize, char)> = vec![];
    let mut counter = 0;
    while true {
        println!("Current grid state:");
        for row in &grid {
            let row_str: String = row.iter().collect();
            println!("{}", row_str);
        }
        let mut can_mine_something = false;
        for row_index in 0..grid.len() {
            for col_index in 0..grid[row_index].len() {
                // process each cell
                let cell = grid[row_index][col_index];
                if cell == '#' {
                        changes.push((row_index, col_index, '1'));
                    }
                if check_neighbors(row_index, col_index, cell, &grid) {
                    can_mine_something = true;
                    if cell.is_digit(10) {
                        let digit = cell.to_digit(10).unwrap();
                        changes.push((row_index, col_index, std::char::from_digit(digit + 1, 10).unwrap()));
                    }
                }
            }
        }
        for (r, c, val) in &changes {
            grid[*r][*c] = *val;
        }
        if !can_mine_something {
            break;
        }
        counter += changes.len();
        changes.clear();

    }
    return counter as i32;
}