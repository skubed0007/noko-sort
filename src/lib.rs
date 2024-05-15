
pub fn sort_ascending(data: &str) -> Vec<i128> {
    let mut splitdat: Vec<i128> = data
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    let mut sorted_values = Vec::new();
    let mut min_index;

    while !splitdat.is_empty() {
        min_index = 0;
        for i in 1..splitdat.len() {
            if splitdat[i] < splitdat[min_index] {
                min_index = i;
            }
        }
        sorted_values.push(splitdat.remove(min_index));
    }
    for (i, _val) in sorted_values.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
    }
    return sorted_values;
}

pub fn sort_descending(data: &str) -> Vec<i128> {
    let mut splitdat: Vec<i128> = data
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    let mut sorted_values = Vec::new();
    let mut min_index;

    while !splitdat.is_empty() {
        min_index = 0;
        for i in 1..splitdat.len() {
            if splitdat[i] > splitdat[min_index] {
                min_index = i;
            }
        }
        sorted_values.push(splitdat.remove(min_index));
    }
    for (i, _val) in sorted_values.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
    }
    return sorted_values;
}
