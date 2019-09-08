use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ISBN(String);

impl ISBN {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ISBNError {
    CheckDigitNotValid,
    FormNotValid,
    CharsetNotValid { c: char },
}

impl FromStr for ISBN {
    type Err = ISBNError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // skip hyphens and whitespaces
        let isbn: String = s
            .chars()
            .filter(|&c| c != '-' && !c.is_whitespace())
            .collect();
        let mut digits = 0;
        for c in (&isbn).chars() {
            if !c.is_digit(10) {
                // any invalid character? (can't use Iterator::any because I want the character)
                return Err(ISBNError::CharsetNotValid { c });
            } else {
                digits += 1;
            }
        }
        // check digits for 10
        if digits == 10 {
            let check: u32 = (&isbn)
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .map(|(i, n)| (10 - i as u32) * n)
                .sum();
            if check % 11 == 0 {
                Ok(ISBN(isbn))
            } else {
                Err(ISBNError::CheckDigitNotValid)
            }
        }
        //check digits for 13
        else if digits == 13 {
            let check: u32 = (&isbn)
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .map(|(i, n)| if i % 2 == 0 { n } else { 3 * n })
                .sum();
            if check % 10 == 0 {
                Ok(ISBN(isbn))
            } else {
                Err(ISBNError::CheckDigitNotValid)
            }
        } else {
            Err(ISBNError::FormNotValid)
        }
    }
}
