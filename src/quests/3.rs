// -1 = surface/air ('.')
// 0 = unmined rock ('#')
// 1+ = mining depth

fn build_grid(notes: &String) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = vec![];
    for line in notes.lines() {
        let row: Vec<i32> = line.chars().map(|c| match c {
            '.' => -1,
            '#' => 0,
            d if d.is_digit(10) => d.to_digit(10).unwrap() as i32,
            _ => -1,
        }).collect();
        grid.push(row);
    }
    grid
}

fn check_neighbors(row_index: usize, col_index: usize, value: i32, grid: &Vec<Vec<i32>>) -> bool {
    let args: Vec<String> = std::env::args().collect();
    let task = &args[1];
    let mut can_mine = true;
    let row_len = grid.len() as isize;
    let col_len = grid[0].len() as isize;
    let mut neighbors: Vec<(isize, isize)> = vec![];
    if task == "3.3" {
        neighbors = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1),  (1, 0), (1, 1),
        ];
    } else {
        neighbors = vec![
            (-1, 0),
            (0, -1), (0, 1),
            (1, 0),
        ];
    }
    for (dr, dc) in neighbors.iter() {
        let new_row = row_index as isize + dr;
        let new_col = col_index as isize + dc;
        if new_row < 0 || new_row >= row_len || new_col < 0 || new_col >= col_len {
            continue;
        }
        if grid[new_row as usize][new_col as usize] != value || value == -1 {
            can_mine = false;
        }
    }
    can_mine
}

pub fn mining_1(notes: String) -> i32 {
    let mut grid = build_grid(&notes);
    let mut changes: Vec<(usize, usize, i32)> = vec![];
    let mut counter = 0;
    loop {
        let mut can_mine_something = false;
        for row_index in 0..grid.len() {
            for col_index in 0..grid[row_index].len() {
                let cell = grid[row_index][col_index];
                // Convert unmined rock to depth 1
                if cell == 0 {
                    changes.push((row_index, col_index, 1));
                }
                // Increment depth if all neighbors match
                if check_neighbors(row_index, col_index, cell, &grid) {
                    can_mine_something = true;
                    if cell > 0 {
                        changes.push((row_index, col_index, cell + 1));
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
    counter as i32
}
