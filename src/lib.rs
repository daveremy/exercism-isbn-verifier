/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    match isbn
        .chars()
        .filter(|c| c != &'-')
        .enumerate()
        .try_fold((false, 0), |acc, (i, c)| fold_isbn_char(acc.1, i, c))
    {
        Ok((valid_length, check_sum)) => {
            if valid_length {
                check_sum % 11 == 0
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

fn fold_isbn_char(acc: usize, i: usize, c: char) -> Result<(bool, usize), ()> {
    let mut valid_length: bool = false;
    if i == 9 {
        valid_length = true;
    }
    if i == 9 && c == 'X' {
        Ok((valid_length, acc + 10 * (10 - i)))
    } else {
        if c.is_numeric() {
            Ok((
                valid_length,
                acc + c.to_digit(10).unwrap() as usize * (10 - i),
            ))
        } else {
            Err(())
        }
    }
}
