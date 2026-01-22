fn build_vector(notes: &String) -> Vec<u32> {
    let num_lines = notes.lines().count();
    let mut nails: Vec<u32> = vec![0; num_lines];
    let mut counter = 0;
    for line in notes.lines() {
        nails[counter] = line.trim().parse::<u32>().unwrap();
        counter += 1;
    }
    nails
}

fn get_min(nails: &Vec<u32>) -> u32 {
    let mut min = u32::MAX;
    for num in nails.iter() {
        if *num < min {
            min = *num;
        }
    }
    min as u32
}

fn get_median(nails: &Vec<u32>) -> u32 {
    let mut sorted_nails = nails.clone();
    sorted_nails.sort();
    let len = sorted_nails.len();
    if len % 2 == 0 {
        (sorted_nails[len / 2 - 1] + sorted_nails[len / 2]) / 2
    } else {
        sorted_nails[len / 2]
    }
}

pub fn smithing_1(notes: String) -> i32 {
    let nails = build_vector(&notes);
    let min = get_min(&nails);
    let mut total = 0;
    for num in nails.iter() {
        total += (*num - min) as i32;
    }
    total
}
pub fn smithing_2(notes: String) -> i32 {
    let nails = build_vector(&notes);
    let median = get_median(&nails);
    let mut total = 0;
    for num in nails.iter() {
        total += (if *num > median { *num - median } else { median - *num }) as i32;
    }
    total
}