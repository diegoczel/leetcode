fn main() {
    /*
    I ->  73 | i -> 105
    V ->  86 | v -> 118
    X ->  88 | x -> 120
    L ->  76 | l -> 108
    C ->  67 | c ->  99
    D ->  68 | d -> 100
    M ->  77 | m -> 109
    */

    roman_to_int("IVXLCDM".to_string());
    //println!();
    //roman_to_int("ivxlcdm".to_string());
}

pub fn roman_to_int(s: String) -> i32 {
    let s_new = s.as_bytes();
    let mut i = 0;

    if s_new.len() == 0 {
        return 0;
    }

    let mut ind = 0;
    while ind < s.len() - 1 {
        let c = s_new[ind] as char;

        if c == 'M' {
            i = i + 1000;
        } else if c == 'D' {
            i = i + 500;
        } else if c == 'C' {
            if ind + 1 < s.len() && s_new[ind + 1] as char == 'M' {
                i = i + 900;
                ind = ind + 2;
                continue;
            } else if ind + 1 < s.len() && s_new[ind + 1] as char == 'D' {
                i = i + 400;
                ind = ind + 2;
                continue;
            } else {
                i = i + 100;
            }
        } else if c == 'L' {
            i = i + 50;
        } else if c == 'X' {
            if ind + 1 < s.len() && s_new[ind + 1] as char == 'C' {
                i = i + 90;
                ind = ind + 2;
                continue;
            } else if ind + 1 < s.len() && s_new[ind + 1] as char == 'L' {
                i = i + 40;
                ind = ind + 2;
                continue;
            } else {
                i = i + 10;
            }
        } else if c == 'V' {
            i = i + 5;
        } else { // 'I'
            if ind + 1 < s.len() && s_new[ind + 1] as char == 'X' {
                i = i + 9;
                ind = ind + 2;
                continue;
            } else if ind + 1 < s.len() && s_new[ind + 1] as char == 'V' {
                i = i + 4;
                ind = ind + 2;
                continue;
            } else {
                i = i + 1;
            }
        }

        ind = ind + 1;
    }

    if ind < s.len() && s_new[ind] as char == 'I' {
        i = i + 1;
    } else if ind < s.len() && s_new[ind] as char == 'V' {
        i = i + 5;
    } else if ind < s.len() && s_new[ind] as char == 'X' {
        i = i + 10;
    } else if ind < s.len() && s_new[ind] as char == 'L' {
        i = i + 50;
    } else if ind < s.len() && s_new[ind] as char == 'C' {
        i = i + 100;
    } else if ind < s.len() && s_new[ind] as char == 'D' {
        i = i + 500;
    } else if ind < s.len() && s_new[ind] as char == 'M' {
        i = i + 1000;
    }

    i
}

#[cfg(test)]
pub mod tests {
    use crate::roman_to_int;

    #[test]
    fn roman_to_int_test1() {
        assert_eq!(roman_to_int("".to_string()), 0);
    }

    #[test]
    fn roman_to_int_test2() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn roman_to_int_test3() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn roman_to_int_test4() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn roman_to_int_test5() {
        assert_eq!(roman_to_int("MMCDXXV".to_string()), 2425);
    }

    #[test]
    fn roman_to_int_test6() {
        assert_eq!(roman_to_int("MDLXX".to_string()), 1570);
    }

    #[test]
    fn roman_to_int_test7() {
        assert_eq!(roman_to_int("MDL".to_string()), 1550);
    }

    #[test]
    fn roman_to_int_test8() {
        assert_eq!(roman_to_int("MMMCC".to_string()), 3200);
    }

    #[test]
    fn roman_to_int_test9() {
        assert_eq!(roman_to_int("D".to_string()), 500);
    }

    #[test]
    fn roman_to_int_test10() {
        assert_eq!(roman_to_int("M".to_string()), 1000);
    }
}
