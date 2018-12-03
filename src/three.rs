use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let claims = read_input();
    let output = count_claim_overlap(&claims);

    println!("{}", output);
}

pub fn part_two() {
}

pub fn read_input() -> Vec<Claim> {
    let stdin = io::stdin();

    let mut claims: Vec<Claim> = Vec::new();
    for line in stdin.lock().lines() {
        let (x1, y1, width, height) = extract_claim_info(&line.unwrap());

        let claim = construct_claim(x1, y1, width, height);

        claims.push(claim);
    }

    claims
}

fn count_claim_overlap(claims: &Vec<Claim>) -> i32 {
    let mut fabric: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

    for claim in claims.iter() {
        apply_claim(&mut fabric, claim);
    }

    count_overlap(&fabric)
}

fn count_overlap(fabric: &[[i8; 1000]; 1000]) -> i32 {
    let mut count = 0;
    for x in 0 .. 1000 {
        for y in 0 .. 1000 {
            let x = x as usize;
            let y = y as usize;

            if fabric[x][y] == 2 {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Claim {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub fn construct_claim(x1: i32, y1: i32, width: i32, height: i32) -> Claim {
    Claim {
        x1,
        y1,
        x2: x1 + width,
        y2: y1 + height,
    }
}

pub fn extract_claim_info(claim_str: &str) -> (i32, i32, i32, i32) {
    let mut id_and_rest = claim_str.split("@");
    id_and_rest.next();

    let non_id = id_and_rest.next().unwrap();

    let mut start_and_dims = non_id.split(":");

    let start = start_and_dims.next().unwrap().trim();
    let dims = start_and_dims.next().unwrap().trim();

    let mut x1_and_y1 = start.split(",");

    let x1: i32 = x1_and_y1.next().unwrap().parse().unwrap();
    let y1: i32 = x1_and_y1.next().unwrap().parse().unwrap();

    let mut width_and_height = dims.split("x");

    let width: i32 = width_and_height.next().unwrap().parse().unwrap();
    let height: i32 = width_and_height.next().unwrap().parse().unwrap();

    (x1, y1, width, height)
}

pub fn apply_claim(fabric: &mut [[i8; 1000]; 1000], claim: &Claim) {
    for x in claim.x1 .. claim.x2 {
        for y in claim.y1 .. claim.y2 {
            let x = x as usize;
            let y = y as usize;

            let prev = fabric[x][y];

            if prev < 2 {
                fabric[x][y] += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_claim_it_works_on_an_odd_case() {
        let actual = construct_claim(2, 3, 5, 9);

        let expected = Claim {
            x1: 2,
            y1: 3,
            x2: 7,
            y2: 12,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn extract_claim_info_it_works_on_an_odd_case() {
        let claim_str = "#1 @ 704,926: 5x4";

        let expected = (704, 926, 5, 4);
        let actual = extract_claim_info(claim_str);

        assert_eq!(actual, expected);
    }
}
