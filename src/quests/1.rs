pub fn total_potions_1(notes: String) -> i32 {
    let mut potions: i32 = 0;
    for note in notes.chars() {
        match note {
            'A' => (),
            'B' => potions += 1,
            'C' => potions += 3,
            _ => (),
        }
    }
    return potions;
}

pub fn total_potions_2(notes: String) -> i32 {
    let mut potions: i32 = 0;
    let mut pair = 0;
    notes.char_indices().for_each(|(index, char)| {
        if index % 2 == 0 {
            pair = 1;
            match char {
                'A' => (),
                'B' => potions += 1,
                'C' => potions += 3,
                'D' => potions += 5,
                'x' => pair = 0,
                _ => (),
            }
        } else {
            match char {
                'A' => (),
                'B' => potions += 1,
                'C' => potions += 3,
                'D' => potions += 5,
                'x' => pair = 0,
                _ => (),
            }
            if pair == 1 {
                potions += 2;
            }
        }
    });
    return potions;
}

pub fn total_potions_3(notes: String) -> i32 {
    let mut potions: i32 = 0;
    let mut triple = 0;
    notes.char_indices().for_each(|(index, char)| {
        triple += 1;
        if (index + 1) % 3 == 0 {
            match char {
                'A' => (),
                'B' => potions += 1,
                'C' => potions += 3,
                'D' => potions += 5,
                'x' => triple -= 1,
                _ => (),
            }
            if triple == 3 {
                potions += 6;
            } else if triple == 2{
                potions += 2;
            }
            triple = 0;
        } else {
            match char {
                'A' => (),
                'B' => potions += 1,
                'C' => potions += 3,
                'D' => potions += 5,
                'x' => triple -= 1,
                _ => (),
            }
            
        }
    });
    return potions;
}