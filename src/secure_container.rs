fn contains_adjacent_digits(number: u32) -> bool {
    //number as string
    //check each character with next until match then break
    let nmb_str = number.to_string();
    let mut iter = nmb_str.chars().peekable();

    while let Some(current_ch) = iter.next() {
        match iter.peek() {
            Some(next_ch) => {
                if current_ch == *next_ch {
                    return true;
                }
            }
            None => continue,
        }
    }
    false
}

pub(crate) fn no_decrease_left_to_right(number: u32) -> bool {
    let nmb_str = number.to_string();
    let mut iter = nmb_str.chars().peekable();
    while let Some(current_ch) = iter.next() {
        match iter.peek() {
            Some(next_ch) => {
                let current_nmb = current_ch
                    .to_digit(10)
                    .expect("How does a number parsed to a string fail when moving back again?");
                let next_nmb = next_ch
                    .to_digit(10)
                    .expect("How does a number parsed to a string fail when moving back again?");
                if next_nmb < current_nmb {
                    return false;
                }
            }
            None => continue,
        }
    }
    true
}

pub(crate) fn six_digits_long(number: u32) -> bool {
    number.to_string().len() == 6
}

pub fn password_check(number: u32) -> bool {
    six_digits_long(number) && contains_adjacent_digits(number) && no_decrease_left_to_right(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_adjacent_digits() {
        let mut number = 1223456;
        assert_eq!(contains_adjacent_digits(number), true);
        number = 123456;
        assert_eq!(contains_adjacent_digits(number), false);
        number = 111111;
        assert_eq!(contains_adjacent_digits(number), true);
    }

    #[test]
    fn test_increase_left_to_right() {
        let mut number = 123456;
        assert_eq!(no_decrease_left_to_right(number), true);
        number = 12355;
        assert_eq!(no_decrease_left_to_right(number), true);
    }

    #[test]
    fn test_password_check() {
        let mut number = 111111;
        assert_eq!(password_check(number), true);
        number = 223450;
        assert_eq!(password_check(number), false);
        number = 123789;
        assert_eq!(password_check(number), false);
    }
}
