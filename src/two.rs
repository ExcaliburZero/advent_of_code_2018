extern crate multimap;

use std::collections::LinkedList;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use self::multimap::MultiMap;

pub fn part_one() {
    let input = read_input();
    let output = checksum_ids(input);

    println!("{}", output);
}

pub fn part_two() {
    let input = read_input();
    let output = find_common_chars_in_1_diff(input).unwrap();

    println!("{}", output);
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

pub fn find_common_chars_in_1_diff(box_ids: LinkedList<String>) -> Option<String> {
    let mut hashes: MultiMap<i32, &str> = MultiMap::new();

    for id in box_ids.iter() {
        for char_i_to_remove in 0..id.len() {
            let remaining_chars = remove_char_at(id, char_i_to_remove);
            let hash = hash_string(remaining_chars);

            hashes.insert(hash, id);

            let possible_matches = hashes.get_vec(&hash);

            match possible_matches {
                Some(possible_matches) => {
                    for possible_match in possible_matches {
                        if off_by_one(id, possible_match) {
                            return Some(common_chars(id, possible_match))
                        }
                    }
                },
                None => {},
            }
        }
    }

    None
}

pub fn off_by_one(str_a: &str, str_b: &str) -> bool {
    let mut diff = 0;

    for (i, c) in str_a.chars().enumerate() {
        let c2 = str_b.as_bytes()[i] as char;   // Assume ascii string

        if c != c2 {
            if diff < 1 {
                diff += 1;
            } else {
                return false
            }
        }
    }

    diff == 1
}

pub fn common_chars(str_1: &str, str_b: &str) -> String {
    str_1.chars().zip(str_b.chars())
        .filter(|(a, b)| a == b)
        .map(|(c, _)| c)
        .collect()
}

pub fn remove_char_at(id: &str, i: usize) -> String {
    let mut chars_to_keep = Vec::new();

    for (j, c) in id.chars().enumerate() {
        if j != i {
            chars_to_keep.push(c);
        }
    }

    chars_to_keep.into_iter().collect()
}

pub fn hash_string(string: String) -> i32 {
    let mut hash = 1;

    for letter in string.chars() {
        let ascii = letter as i32;

        hash += ascii;
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_letter_counts_it_works_on_empty_string() {
        let id = "".to_string();

        let counts = calc_letter_counts(id);

        assert_eq!(counts.len(), 0);
    }

    #[test]
    fn calc_letter_counts_it_works_on_string_with_no_repeats() {
        let id = "abcd".to_string();

        let counts = calc_letter_counts(id);

        assert_eq!(counts.len(), 4);

        assert_eq!(counts.get(&'a'), Some(&1));
        assert_eq!(counts.get(&'b'), Some(&1));
        assert_eq!(counts.get(&'c'), Some(&1));
        assert_eq!(counts.get(&'d'), Some(&1));
    }

    #[test]
    fn calc_letter_counts_it_works_on_string_with_multiple_repeats() {
        let id = "aabcbda".to_string();

        let counts = calc_letter_counts(id);

        assert_eq!(counts.len(), 4);

        assert_eq!(counts.get(&'a'), Some(&3));
        assert_eq!(counts.get(&'b'), Some(&2));

        assert_eq!(counts.get(&'c'), Some(&1));
        assert_eq!(counts.get(&'d'), Some(&1));
    }

    #[test]
    fn remove_char_at_it_works_on_first_letter() {
        let string = "abcd";
        let i = 0;

        let new_string = remove_char_at(string, i);

        assert_eq!(new_string, "bcd".to_string());
    }

    #[test]
    fn remove_char_at_it_works_on_intermediate_letter() {
        let string = "abcd";
        let i = 2;

        let new_string = remove_char_at(string, i);

        assert_eq!(new_string, "abd".to_string());
    }

    #[test]
    fn remove_char_at_it_works_on_last_letter() {
        let string = "abcd";
        let i = 3;

        let new_string = remove_char_at(string, i);

        assert_eq!(new_string, "abc".to_string());
    }

    #[test]
    fn hash_string_it_works_on_empty_string() {
        let string = "".to_string();

        let hash = hash_string(string);

        assert_eq!(hash, 1);
    }

    #[test]
    fn hash_string_it_works_on_nonempty_string() {
        let string = "abac".to_string();

        let hash = hash_string(string);

        assert_eq!(hash, 392);
    }
}
