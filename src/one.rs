use std::collections::LinkedList;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let changes = read_input();
    let answer = sum_changes(&changes);

    println!("{}", answer);
}

pub fn part_two() {
    let changes = read_input();
    let answer = get_first_repeat_frequency(&changes);

    println!("{}", answer);
}

/// Reads in the input lines and converts them to a LinkedList of i32 values.
pub fn read_input() -> LinkedList<i32> {
    let stdin = io::stdin();

    let mut changes: LinkedList<i32> = LinkedList::new();
    for line in stdin.lock().lines() {
        let change: i32 = line.unwrap().parse().unwrap();

        changes.push_back(change);
    }

    changes
}

/// Sums up the provided changes and returns the resulting frequency.
///
/// ```
/// use std::collections::LinkedList;
/// use advent_of_code_2018::one;
///
/// let mut changes = LinkedList::new();
///
/// changes.push_back(2);
/// changes.push_back(1);
/// changes.push_back(-4);
/// changes.push_back(5);
///
/// let res = one::sum_changes(&changes);
///
/// assert_eq!(res, 4);
/// ```
pub fn sum_changes(changes: &LinkedList<i32>) -> i32 {
    changes.iter().sum()
}

/// Repeatedly re-sums up the given changes and returns the first repeated
/// frequency.
///
/// If no repeats ever occur, then this function will not halt.
///
/// ```
/// use std::collections::LinkedList;
/// use advent_of_code_2018::one;
///
/// let mut changes = LinkedList::new();
///
/// changes.push_back(2);
/// changes.push_back(1);
/// changes.push_back(-1);
/// changes.push_back(5);
///
/// let res = one::get_first_repeat_frequency(&changes);
///
/// assert_eq!(res, 2);
/// ```
pub fn get_first_repeat_frequency(changes: &LinkedList<i32>) -> i32 {
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut sum = 0;

    loop {
        for change in changes.iter() {
            sum += change;

            if frequencies.contains(&sum) {
                return sum;
            } else {
                frequencies.insert(sum);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::LinkedList;

    #[test]
    fn sum_changes_it_sums_zero_changes() {
        let changes = LinkedList::new();
        let res = sum_changes(&changes);

        assert_eq!(res, 0);
    }

    #[test]
    fn sum_changes_it_sums_one_change() {
        let mut changes = LinkedList::new();

        changes.push_back(2);

        let res = sum_changes(&changes);

        assert_eq!(res, 2);
    }

    #[test]
    fn sum_changes_it_sums_multiple_changes() {
        let mut changes = LinkedList::new();

        changes.push_back(2);
        changes.push_back(1);
        changes.push_back(-4);
        changes.push_back(5);

        let res = sum_changes(&changes);

        assert_eq!(res, 4);
    }

    #[test]
    fn get_first_repeat_frequency_it_finds_repeat_on_partial_iteration() {
        let mut changes = LinkedList::new();

        changes.push_back(2);
        changes.push_back(1);
        changes.push_back(-1);
        changes.push_back(5);

        let res = get_first_repeat_frequency(&changes);

        assert_eq!(res, 2);
    }

    #[test]
    fn get_first_repeat_frequency_it_finds_repeat_on_second_iteration() {
        let mut changes = LinkedList::new();

        changes.push_back(-1);
        changes.push_back(1);
        changes.push_back(2);

        let res = get_first_repeat_frequency(&changes);

        assert_eq!(res, 2);
    }
}
