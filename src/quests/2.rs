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
    return runic_words as i32;
}

pub fn runic_words_2(notes: String) -> i32 {
    let mut runic_words = 0;
    // let mut debug_counter = 0;
    let (words, set, smallest, largest) = parser(&notes);
    for word in words {
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
                let sub_back = &word[left..right].chars().rev().collect::<String>();
                if set.contains(sub) || set.contains(sub_back) {
                    for i in left..right {
                        if indices.contains(&(i)) == false {
                            indices.push(i);
                        }
                    }
                }
                left += 1;
                right += 1;
            }
            // if debug_counter < 50 {
            //     println!("Word '{}', size {}, indices {:?}, count {}", word, size, indices, indices.len());
            // }
            // debug_counter += 1;
        }
        runic_words += indices.len() as i32;
    }
    return runic_words as i32;
}

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
    return (words, set, smallest, largest);
}
