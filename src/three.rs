use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let claims = read_input();
    let output = count_claim_overlap(&claims);

    println!("{}", output);
}

pub fn part_two() {
    let claims = read_input();
    let output = find_non_overlapping_claim(&claims).unwrap();

    println!("{}", output);
}

pub fn read_input() -> Vec<Claim> {
    let stdin = io::stdin();

    let mut claims: Vec<Claim> = Vec::new();
    for line in stdin.lock().lines() {
        let (id, x1, y1, width, height) = extract_claim_info(&line.unwrap());

        let claim = construct_claim(id, x1, y1, width, height);

        claims.push(claim);
    }

    claims
}

/// Returns the number of cells in the 1000 by 1000 fabric that are claimed by
/// more than one of the given claims.
fn count_claim_overlap(claims: &[Claim]) -> i32 {
    let mut fabric: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

    for claim in claims.iter() {
        apply_claim(&mut fabric, claim);
    }

    count_overlap(&fabric)
}

/// Returns the number of cells in the given fabric grid that have had multiple
/// claims applied to them. All of the claims must have already been applied to
/// the fabric grid.
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
    id: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub fn construct_claim(id: i32, x1: i32, y1: i32, width: i32, height: i32) -> Claim {
    Claim {
        id,
        x1,
        y1,
        x2: x1 + width,
        y2: y1 + height,
    }
}

/// Extracts the claim information from the given claim description string.
pub fn extract_claim_info(claim_str: &str) -> (i32, i32, i32, i32, i32) {
    let mut id_and_rest = claim_str.split('@');

    let mut id = id_and_rest.next().unwrap().trim().to_string();
    id.remove(0);

    let id = id.parse().unwrap();

    let non_id = id_and_rest.next().unwrap();

    let mut start_and_dims = non_id.split(':');

    let start = start_and_dims.next().unwrap().trim();
    let dims = start_and_dims.next().unwrap().trim();

    let mut x1_and_y1 = start.split(',');

    let x1: i32 = x1_and_y1.next().unwrap().parse().unwrap();
    let y1: i32 = x1_and_y1.next().unwrap().parse().unwrap();

    let mut width_and_height = dims.split('x');

    let width: i32 = width_and_height.next().unwrap().parse().unwrap();
    let height: i32 = width_and_height.next().unwrap().parse().unwrap();

    (id, x1, y1, width, height)
}

/// Applies the given claim to the given fabric grid, setting all cells for the
/// claim to 1, unless it is already covered by a different claim in which case
/// it is set to 2.
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

/// Finds the first of the given claims that does not overlap with any other
/// claim.
pub fn find_non_overlapping_claim(claims: &[Claim]) -> Option<i32> {
    let mut fabric: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

    for claim in claims.iter() {
        apply_claim(&mut fabric, claim);
    }

    for claim in claims.iter() {
        if claim_is_non_overlapping(&fabric, claim) {
            return Some(claim.id)
        }
    }

    None
}

/// Checks if the given claim doesnt overlap with any other claim, by looking
/// at a fabric grid that has already had all claims applied to it.
pub fn claim_is_non_overlapping(fabric: &[[i8; 1000]; 1000], claim: &Claim) -> bool {
    for x in claim.x1 .. claim.x2 {
        for y in claim.y1 .. claim.y2 {
            let x = x as usize;
            let y = y as usize;

            if fabric[x][y] == 2 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_claim_overlap_it_works_on_a_case_with_no_claims() {
        let claims = vec![];
        let actual = count_claim_overlap(&claims);

        assert_eq!(actual, 0);
    }

    #[test]
    fn count_claim_overlap_it_works_on_a_case_with_overlap() {
        let claim1 = Claim {
            id: 1,
            x1: 1,
            y1: 1,
            x2: 5,
            y2: 5,
        };

        let claim2 = Claim {
            id: 2,
            x1: 2,
            y1: 2,
            x2: 5,
            y2: 5,
        };

        let claims = vec![claim1, claim2];

        let actual = count_claim_overlap(&claims);

        assert_eq!(actual, 9);
    }

    #[test]
    fn count_overlap_it_works_on_a_case_with_overlap() {
        let mut fabric: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

        let claim1 = Claim {
            id: 1,
            x1: 1,
            y1: 1,
            x2: 5,
            y2: 5,
        };

        let claim2 = Claim {
            id: 2,
            x1: 2,
            y1: 2,
            x2: 5,
            y2: 5,
        };

        apply_claim(&mut fabric, &claim1);
        apply_claim(&mut fabric, &claim2);

        let actual = count_overlap(&fabric);

        assert_eq!(actual, 9);
    }


    #[test]
    fn construct_claim_it_works_on_an_odd_case() {
        let actual = construct_claim(1, 2, 3, 5, 9);

        let expected = Claim {
            id: 1,
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

        let expected = (1, 704, 926, 5, 4);
        let actual = extract_claim_info(claim_str);

        assert_eq!(actual, expected);
    }

    #[test]
    fn apply_claim_it_works_on_a_simple_case() {
        let mut fabric: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

        let claim = Claim {
            id: 1,
            x1: 2,
            y1: 3,
            x2: 7,
            y2: 12,
        };

        apply_claim(&mut fabric, &claim);

        for x in 0 .. 1000 {
            for y in 0 .. 1000 {
                if x >= claim.x1 && x < claim.x2 && y >= claim.y1 &&
                    y < claim.y2 {
                    let x = x as usize;
                    let y = y as usize;

                    assert_eq!(fabric[x][y], 1);
                } else {
                    let x = x as usize;
                    let y = y as usize;

                    assert_eq!(fabric[x][y], 0);
                }
            }
        }
    }

    #[test]
    fn apply_claim_it_works_on_a_case_with_overlap() {
        let mut fabric: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

        let claim1 = Claim {
            id: 1,
            x1: 5,
            y1: 7,
            x2: 7,
            y2: 12,
        };

        let claim2 = Claim {
            id: 2,
            x1: 2,
            y1: 3,
            x2: 7,
            y2: 12,
        };

        apply_claim(&mut fabric, &claim1);
        apply_claim(&mut fabric, &claim2);

        for x in 0 .. 1000 {
            for y in 0 .. 1000 {
                let in_claim1 = x >= claim1.x1 && x < claim1.x2 && y >= claim1.y1
                    && y < claim1.y2;
                let in_claim2 = x >= claim2.x1 && x < claim2.x2 && y >= claim2.y1
                    && y < claim2.y2;

                let expected = if in_claim1 && in_claim2 {
                    2
                } else if in_claim1 || in_claim2 {
                    1
                } else {
                    0
                };

                let x = x as usize;
                let y = y as usize;

                assert_eq!(fabric[x][y], expected);
            }
        }
    }

    #[test]
    fn find_non_overlapping_claim_it_works_on_all_overlapping_claims() {
        let claim1 = Claim {
            id: 1,
            x1: 1,
            y1: 1,
            x2: 5,
            y2: 5,
        };

        let claim2 = Claim {
            id: 2,
            x1: 2,
            y1: 2,
            x2: 5,
            y2: 5,
        };

        let claims = vec![claim1, claim2];

        let actual = find_non_overlapping_claim(&claims);

        assert_eq!(actual, None);
    }

    #[test]
    fn find_non_overlapping_claim_it_works_on_a_case_with_a_non_overlapping_claim() {
        let claim1 = Claim {
            id: 1,
            x1: 1,
            y1: 1,
            x2: 5,
            y2: 5,
        };

        let claim2 = Claim {
            id: 2,
            x1: 2,
            y1: 2,
            x2: 5,
            y2: 5,
        };

        let claim3 = Claim {
            id: 3,
            x1: 9,
            y1: 9,
            x2: 19,
            y2: 19,
        };

        let claims = vec![claim1, claim2, claim3];

        let actual = find_non_overlapping_claim(&claims);

        assert_eq!(actual, Some(3));
    }

    #[test]
    fn claim_is_non_overlapping_it_works_on_an_overlapping_claim() {
        let mut fabric: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

        let claim1 = Claim {
            id: 1,
            x1: 1,
            y1: 1,
            x2: 5,
            y2: 5,
        };

        let claim2 = Claim {
            id: 2,
            x1: 2,
            y1: 2,
            x2: 5,
            y2: 5,
        };

        apply_claim(&mut fabric, &claim1);
        apply_claim(&mut fabric, &claim2);

        let actual = claim_is_non_overlapping(&fabric, &claim1);

        assert_eq!(actual, false);
    }

    #[test]
    fn claim_is_non_overlapping_it_works_on_a_non_overlapping_claim() {
        let mut fabric: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

        let claim1 = Claim {
            id: 1,
            x1: 1,
            y1: 1,
            x2: 5,
            y2: 5,
        };

        let claim2 = Claim {
            id: 2,
            x1: 20,
            y1: 20,
            x2: 25,
            y2: 25,
        };

        apply_claim(&mut fabric, &claim1);
        apply_claim(&mut fabric, &claim2);

        let actual = claim_is_non_overlapping(&fabric, &claim1);

        assert_eq!(actual, true);
    }
}
