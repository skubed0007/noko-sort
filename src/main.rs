use std::{io::{stdin, stdout, Write}, process::exit};

fn main() {
    let mut dat = String::new();
    println!("PLEASE PUT THE DATA SEPARATED BY STRING");
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut dat).unwrap();
    let mut chc = String::new();
    println!("what do want me to do?\n'de' for descending order\n'as' for ascending order");
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut chc).unwrap();
    if chc.trim().to_lowercase() == "as" {
        sort_ascending(dat.trim());
    } else if chc.trim().to_lowercase() == "de" {
        sort_descending(dat.trim());
    }
    else{
        eprintln!("please choose correct option\neither 'as' or 'de'\n...");
        exit(1);
    }
}

fn sort_ascending(data: &str) {
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
    for (i, val) in sorted_values.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}

fn sort_descending(data: &str) {
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
    for (i, val) in sorted_values.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}
