use crate::secure_container::{no_decrease_left_to_right, six_digits_long};
use std::collections::HashMap;

fn at_least_one_pair_adjacent_digits_not_in_larger_group(number: u32) -> bool {
    let nmb_str = number.to_string();
    let mut iter = nmb_str.chars().peekable();
    let mut grouping = HashMap::new();
    while let Some(current_ch) = iter.next() {
        match iter.peek() {
            Some(next_ch) => {
                if current_ch == *next_ch {
                    grouping
                        .entry(current_ch)
                        .and_modify(|nmb| *nmb += 1)
                        .or_insert(2);
                }
            }
            None => continue,
        }
    }
    for value in grouping.values() {
        if *value == 2 {
            return true;
        }
    }
    false
}

pub fn password_check(number: u32) -> bool {
    six_digits_long(number)
        && at_least_one_pair_adjacent_digits_not_in_larger_group(number)
        && no_decrease_left_to_right(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_check() {
        let mut number = 112233;
        assert_eq!(password_check(number), true);
        number = 123444;
        assert_eq!(password_check(number), false);
        number = 111122;
        assert_eq!(password_check(number), true);
        number = 112223;
        assert_eq!(password_check(number), true);
        number = 123455;
        assert_eq!(password_check(number), true);
    }
}
