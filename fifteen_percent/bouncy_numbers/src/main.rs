fn main() {
    let start: i64 = 1;
    let mut bouncy: i64 = 0;
    let mut curr_num = start;
    let mut printed = 0;
    loop {
        if is_bouncy(curr_num) {
            bouncy += 1;
        }
        let perc = bouncy as f64 / curr_num as f64 * 100.0;
        if (printed == 2 && perc >= 99.0)
            || (printed == 1 && perc >= 90.0)
            || (printed == 0 && perc >= 50.0)
        {
            println!("{curr_num} {perc}%");
            printed += 1;
            if printed == 3 {
                break;
            }
        }
        curr_num += 1;

    }
}

fn is_bouncy(num: i64) -> bool {
    if num < 100 {
        return false;
    } else {
        return !(is_descending(num) || is_ascending(num));
    }
}

fn is_single_digit_number(num: i64) -> bool {
    let first_digit = num % 10;
    num.to_string()
        .chars()
        .all(|c| c.to_digit(10).unwrap() == first_digit as u32)
}

fn is_ascending(mut num: i64) -> bool {
    if num < 10 {
        return true;
    }
    // if is_single_digit_number(num){
    //     return false;
    // }
    let mut prev_digit: u8 = (num % 10) as u8;
    while num >= 10 {
        num /= 10;
        let digit = (num % 10) as u8;
        if digit > prev_digit {
            return false;
        }
        prev_digit = digit;
    }
    return true;
}

fn is_descending(mut num: i64) -> bool {
    if num < 10 {
        return true;
    }
    // if is_single_digit_number(num){
    //     return false;
    // }
    let mut prev_digit: u8 = (num % 10) as u8;
    while num > 10 {
        num /= 10;
        let digit = (num % 10) as u8;
        if digit < prev_digit {
            return false;
        }
        prev_digit = digit;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_descending() {
        assert!(is_descending(987654321));
        assert!(is_descending(987));
        assert!(is_descending(7654));
        assert!(is_descending(3210));
        assert!(is_descending(21));
        assert!(is_descending(66420));
        assert!(is_descending(44));
        assert!(is_descending(444));
        assert!(is_descending(1));
        assert!(!is_descending(101));
        assert!(!is_descending(123456));
        assert!(!is_descending(123));
        assert!(!is_descending(76569));
    }

    #[test]
    fn test_is_ascending() {
        assert!(!is_ascending(987654321));
        assert!(!is_ascending(987));
        assert!(!is_ascending(7654));
        assert!(!is_ascending(3210));
        assert!(!is_ascending(21));
        assert!(!is_ascending(76567));
        assert!(!is_ascending(1011));
        assert!(is_ascending(123456));
        assert!(is_ascending(345));
        assert!(is_ascending(56789));
        assert!(is_ascending(123));
        assert!(is_ascending(134468));
        assert!(is_ascending(44));
        assert!(is_ascending(444));
        assert!(is_ascending(1));
    }

    #[test]
    fn test_is_bouncy() {
        assert!(!is_bouncy(987654321));
        assert!(!is_bouncy(987));
        assert!(!is_bouncy(3210));
        assert!(!is_bouncy(21));
        assert!(!is_bouncy(123456));
        assert!(!is_bouncy(56789));
        assert!(!is_bouncy(99));
        assert!(!is_bouncy(1));
        assert!(!is_bouncy(444));
        assert!(is_bouncy(876567));
        assert!(is_bouncy(143));
        assert!(is_bouncy(6068));
    }

    #[test]
    fn test_is_single_digit_number() {
        assert!(is_single_digit_number(11));
        assert!(is_single_digit_number(2222));
        assert!(is_single_digit_number(55555555));
        assert!(!is_single_digit_number(123));
        assert!(!is_single_digit_number(111112));
        assert!(!is_single_digit_number(211111));
    }
}
