fn int_to_roman(num: i32) -> String {
    let mut roman_str = String::new();

    let mut n = num;
    while n > 0 {
        match n {
            1000..=3999 => {
                n -= 1000;
                roman_str += "M";
            },
            900..=999 => {
                n -= 900;
                roman_str += "CM";
            },
            500..=899 => {
                n -= 500;
                roman_str += "D";
            },
            400..=499 => {
                n -= 400;
                roman_str += "CD";
            },
            100..=399 => {
                n -= 100;
                roman_str += "C";
            },
            90..=99 => {
                n -= 90;
                roman_str += "XC";
            },
            50..=89 => {
                n -= 50;
                roman_str += "L";
            },
            40..=49 => {
                n -= 40;
                roman_str += "XL";
            },
            10..=39 => {
                n -= 10;
                roman_str += "X";
            },
            9..=9 => {
                n -= 9;
                roman_str += "IX";
            },
            5..=8 => {
                n -= 5;
                roman_str += "V";
            },
            4..=4 => {
                n -= 4;
                roman_str += "IV";
            },
            1..=3 => {
                n -= 1;
                roman_str += "I";
            }
            _ => {}
        }
    }

    roman_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(int_to_roman(3), "III".to_string());
        assert_eq!(int_to_roman(58), "LVIII".to_string());
        assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX".to_string());
    }
}