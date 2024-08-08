static ENGLISH_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub fn is_alphabetic(char: &str) -> bool {
    ENGLISH_ALPHABET.contains(char) | "_{}".contains(char)
}

pub fn is_numeric(char: &str) -> bool {
    "0123456789i.".contains(char)
}

pub fn is_operational(char: &str) -> bool {
    "+-*/^()".contains(char)
}

pub fn is_variable_id(input: &str) -> bool {
    let mut flag: [bool; 4] = [false, false, false, false];
    for i in input.chars() {
        if ENGLISH_ALPHABET.contains(i) {
            if flag != [true, false, false, false]
                && flag != [false, false, false, false]
                && flag != [true, true, true, false]
            {
                return false;
            }
            flag[0] = true;
        } else {
            match i {
                '_' => {
                    if flag != [true, false, false, false] {
                        return false;
                    }
                    flag[1] = true;
                }
                '{' => {
                    if flag != [true, true, false, false] {
                        return false;
                    }
                    flag[2] = true;
                }
                '}' => {
                    if flag != [true, true, true, false] {
                        return false;
                    }
                    flag[3] = true;
                }
                _ => {
                    if flag != [true, true, true, false] {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

pub fn is_number(input: &str) -> bool {
    let mut decimal: bool = false;
    let mut imaginary: bool = false;
    for i in input.chars() {
        if !"0123456789".contains(i) && !imaginary {
            match i {
                '.' => {
                    if decimal {
                        return false;
                    }
                    decimal = true;
                }
                'i' => {
                    imaginary = true;
                }
                _ => return false,
            }
        } else {
        }
    }
    return true;
}
