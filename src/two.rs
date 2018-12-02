use std::collections::LinkedList;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let input = read_input();
    let output = checksum_ids(input);

    println!("{}", output);
}

pub fn part_two() {
}

pub fn read_input() -> LinkedList<String> {
    let stdin = io::stdin();

    let mut box_ids: LinkedList<String> = LinkedList::new();
    for line in stdin.lock().lines() {
        let id = line.unwrap();

        // Ignore the last line, since it is empty
        if id != "" {
            box_ids.push_back(id);
        }
    }

    box_ids
}

pub fn checksum_ids(box_ids: LinkedList<String>) -> i32 {
    let mut two_counts = 0;
    let mut three_counts = 0;

    for id in box_ids.iter() {
        let counts = calc_letter_counts(id.to_string());

        let mut has_two = false;
        let mut has_three = false;
        for value in counts.values() {
            let v: i32 = *value;

            if v == 2 {
                has_two = true;
            } else if v == 3 {
                has_three = true;
            }
        }

        if has_two {
            two_counts += 1;
        }
        if has_three {
            three_counts += 1;
        }
    }

    two_counts * three_counts
}

pub fn calc_letter_counts(id: String) -> HashMap<char, i32> {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for letter in id.chars() {
        counts.entry(letter)
            .and_modify(|c| { *c += 1 })
            .or_insert(1)
        ;
    }

    counts
}
