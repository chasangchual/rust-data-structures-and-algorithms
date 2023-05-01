use std::io;
use std::io::Write;
use std::ops::Add;

pub fn validate_credit_card_number() -> bool {
    let mut input = String::new();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    stdout.write_all(b"Enter credit card number to validate: ");
    stdout.flush();

    stdin.read_line(&mut input);
    let mut credit_card_number = trim_space(&mut input);
    let is_numeric: bool = is_all_numeric(&mut credit_card_number);
    let mut is_valid = false;
    if is_numeric {
        is_valid = luhn_validation(&mut credit_card_number);
        println!("{} is {}", input.trim_end(), if is_valid { "valid"} else {"not valid"});
    } else {
        println!("{} should be numeric.", credit_card_number);
    }
    is_valid
}

/**
    validate the credit card number
    ref: https://www.groundlabs.com/blog/anatomy-of-a-credit-card/#:~:text=The%20final%20digits%20of%20your,to%20validate%20primary%20account%20numbers.
*/
fn luhn_validation(credit_card_number: &mut String) -> bool {
    let mut digits:Vec<u32> = Vec::new();
    let mut numbers: Vec<_> = credit_card_number.chars().collect();
    let len = credit_card_number.len() - 1;
    let mut is_valid = false;
    let flag = if len % 2 == 0 {1} else {0};

    for i in 0 .. len {
        let c = numbers.get(i);
        let n = c.unwrap().to_digit(10).unwrap();
        if i  % 2 == flag {
            let doubled = n * 2;
            if doubled >= 10 {
                digits.push(doubled - 9);
            } else {
                digits.push(doubled);
            }
        } else {
            // it assumes, the 'c' is already verified as a decimal number
            digits.push(n);
        }
    }

    let mut sum = 0;
    for v in digits {
        sum += v;
    }

    let check_digit = numbers.get(len).unwrap().to_digit(10).unwrap();
    is_valid = 10 - (sum % 10) == if check_digit != 0 {check_digit} else {10};
    is_valid
}

fn trim_space(in_value: &mut String) -> String {
    let mut trimmed = String::new();

    for c in in_value.chars() {
        if !c.is_ascii_whitespace() {
            trimmed.push(c);
        }
    }
    trimmed
}

pub fn is_all_numeric(in_value: &mut String) -> bool {
    for c in in_value.chars() {
        if !c.is_ascii_digit() {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    static visa_valid_card_numbers: [&str; 3] = ["4556 1062 7579 9778", "4716 4406 0243 3257", "4716 0322 4157 1264"];
    static visa_invalid_card_numbers: [&str; 3] = ["4556 1062 7679 9778", "4726 4406 0243 3257", "4716 0322 4157 1263"];

    static master_valid_card_numbers: [&str; 3] = ["5267 9875 4815 4322", "5429 3879 9678 8066", "5191 3189 9111 3444"];
    static master_invalid_card_numbers: [&str; 3] = ["5267 9875 4825 4322", "5429 3899 9678 8066", "5191 2189 9111 3444"];

    static amex_valid_card_numbers: [&str; 3] = ["3768 7346 7451 593", "3415 3077 1696 790", "3770 6705 862 7427"];
    static amex_invalid_card_numbers: [&str; 3] = ["3768 7349 7451 593", "3425 3077 1696 790", "3778 6705 8627 427"];

    #[test]
    fn test_is_all_numeric_with_number_string() {
        let mut trimmed:String = trim_space(&mut String::from(visa_valid_card_numbers[0]));
        let is_numeric:bool = is_all_numeric(&mut trimmed);
        assert_eq!(true, is_numeric);
    }

    #[test]
    fn test_is_all_numeric_with_non_number_string() {
        let mut card_number = String::from( "45561062757997ABC");
        let is_numeric:bool = is_all_numeric(&mut card_number);
        assert_eq!(false, is_numeric);
    }

    #[test]
    fn test_is_trim_space() {
        let mut card_number = String::from( "4556 1062 7579 9778");
        let trimmed:String = trim_space(&mut card_number);
        assert_eq!("4556106275799778", trimmed);
    }

    #[test]
    fn test_validate_visa_card_numbers() {
        for card_number in visa_valid_card_numbers {
            let mut trimmed:String = trim_space(&mut String::from(card_number));
            let mut is_valid = luhn_validation(&mut trimmed);
            assert_eq!(true, is_valid);
        }
    }

    #[test]
    fn test_validate_corrupted_visa_card_numbers() {
        for card_number in visa_invalid_card_numbers {
            let mut trimmed:String = trim_space(&mut String::from(card_number));
            let mut is_valid = luhn_validation(&mut trimmed);
            assert_eq!(false, is_valid);
        }
    }

    #[test]
    fn test_validate_master_card_numbers() {
        for card_number in master_valid_card_numbers {
            let mut trimmed:String = trim_space(&mut String::from(card_number));
            let mut is_valid = luhn_validation(&mut trimmed);
            assert_eq!(true, is_valid);
        }
    }

    #[test]
    fn test_validate_corrupted_master_card_numbers() {
        for card_number in master_invalid_card_numbers {
            let mut trimmed:String = trim_space(&mut String::from(card_number));
            let mut is_valid = luhn_validation(&mut trimmed);
            println!("{} is {}", trimmed, if is_valid { "valid"} else {"not valid"});
            assert_eq!(false, is_valid);
        }
    }

    #[test]
    fn test_validate_amex_card_numbers() {
        for card_number in amex_valid_card_numbers {
            let mut trimmed:String = trim_space(&mut String::from(card_number));
            let mut is_valid = luhn_validation(&mut trimmed);
            assert_eq!(true, is_valid);
        }
    }

    #[test]
    fn test_validate_corrupted_amex_card_numbers() {
        for card_number in amex_invalid_card_numbers {
            let mut trimmed:String = trim_space(&mut String::from(card_number));
            let mut is_valid = luhn_validation(&mut trimmed);
            assert_eq!(false, is_valid);
        }
    }
}





