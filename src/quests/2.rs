use std::collections::HashSet;

pub fn runic_words_1(notes: String) -> i32 {
    let mut runic_words = 0;
    let (words, set, smallest, largest) = parser(&notes);
    for size in smallest..=largest {
        for word in &words {
            let len = word.len();
            if len < size {
                continue;
            }
            let mut left = 0;
            let mut right = size;
            while right <= len {
                let sub = &word[left..right];
                if set.contains(sub) {
                    runic_words += 1;
                }
                left += 1;
                right += 1;
            }
        }
    }
    runic_words as i32
}

pub fn runic_words_2(notes: String) -> i32 {
    let (words, set, smallest, largest) = parser(&notes);
    let mut runic_words = 0;
    for word in &words {
        let indices = find_runic_indices(word, &set, smallest, largest);
        runic_words += indices.len() as i32;
    }
    runic_words
}

// Helper for runic_words_2: finds indices within a single word
fn find_runic_indices(word: &str, set: &HashSet<String>, smallest: usize, largest: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];
    let len = word.len();
    for size in smallest..=largest {
        if len < size {
            continue;
        }
        let mut left = 0;
        let mut right = size;
        while right <= len {
            let sub = &word[left..right];
            let sub_back: String = word[left..right].chars().rev().collect();
            if set.contains(sub) || set.contains(&sub_back) {
                for i in left..right {
                    if !indices.contains(&i) {
                        indices.push(i);
                    }
                }
            }
            left += 1;
            right += 1;
        }
    }
    indices
}

pub fn runic_words_3(notes: String) -> i32 {
    let (grid, set, smallest, largest) = parser_grid(&notes);
    let mut marked: HashSet<(usize, usize)> = HashSet::new();

    let num_rows = grid.len();
    let num_cols = grid.first().map(|r| r.len()).unwrap_or(0);

    // Search horizontally WITH WRAPPING
    for (row, line) in grid.iter().enumerate() {
        let len = line.len();
        if len == 0 {
            continue;
        }
        for size in smallest..=largest {
            if len < size {
                continue;
            }
            // Check all starting positions 
            for col in 0..len {
                let sub: String = (0..size)
                    .map(|i| line[(col + i) % len])
                    .collect();
                let sub_rev: String = sub.chars().rev().collect();
                if set.contains(&sub) || set.contains(&sub_rev) {
                    for i in 0..size {
                        marked.insert((row, (col + i) % len));
                    }
                }
            }
        }
    }

    // Search vertically NO wrapping
    for col in 0..num_cols {
        for size in smallest..=largest {
            if num_rows < size {
                continue;
            }
            for row in 0..=num_rows - size {
                let sub: String = (row..row + size).map(|r| grid[r][col]).collect();
                let sub_rev: String = sub.chars().rev().collect();
                if set.contains(&sub) || set.contains(&sub_rev) {
                    for r in row..row + size {
                        marked.insert((r, col));
                    }
                }
            }
        }
    }

    marked.len() as i32
}

// Parser for runic_words_1 and runic_words_2
fn parser(notes: &String) -> (Vec<String>, HashSet<String>, usize, usize) {
    let mut set: HashSet<String> = HashSet::new();
    let set_words = notes
        .lines()
        .next()
        .unwrap_or("")
        .strip_prefix("WORDS:")
        .unwrap_or("");
    for word in set_words.split(",") {
        let trimmed = word.trim();
        set.insert(trimmed.to_string());
    }
    let mut words: Vec<String> = Vec::new();
    for line in notes.lines().skip(1) {
        for word in line.trim().split(' ') {
            words.push(word.to_string());
        }
    }
    let smallest = set.iter().map(|w| w.len()).min().unwrap_or(0);
    let largest = set.iter().map(|w| w.len()).max().unwrap_or(0);
    (words, set, smallest, largest)
}

// Parser for runic_words_3
fn parser_grid(notes: &String) -> (Vec<Vec<char>>, HashSet<String>, usize, usize) {
    let mut set: HashSet<String> = HashSet::new();
    let mut lines = notes.lines();

    let set_words = lines
        .next()
        .unwrap_or("")
        .strip_prefix("WORDS:")
        .unwrap_or("");
    for word in set_words.split(",") {
        set.insert(word.trim().to_string());
    }

    let grid: Vec<Vec<char>> = lines
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let smallest = set.iter().map(|w| w.len()).min().unwrap_or(0);
    let largest = set.iter().map(|w| w.len()).max().unwrap_or(0);

    (grid, set, smallest, largest)
}
