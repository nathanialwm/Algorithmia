use std::collections::HashSet;
pub fn runic_words_1 (notes: String) -> i32 {
    let mut set: HashSet<String> = HashSet::new();
    let mut runic_words = 0;
    let words = notes.lines()
    .next().unwrap_or("")
    .strip_prefix("WORDS:").unwrap_or("");
    for word in words.split(",") {
        let trimmed = word.trim();
        set.insert(trimmed.to_string());
    }
    for line in notes.lines().skip(1) {
        let words_this_line = line.trim().split(" ");
        let mut window_size = set.iter().map(|s| s.len()).max().unwrap_or(0);
        let mut left = 0;
        let mut right = window_size;
        let mut s = String::new();
        while window_size > 1 as usize {
            while right <= line.len() {
                s = line[left..right].to_string();
                if set.contains(&s) {
                    runic_words += 1;
                }
                left += 1;
                right += 1;
            }
            window_size -= 1;
            left = 0;
            right = window_size;
        }
    }
    return runic_words as i32;
}