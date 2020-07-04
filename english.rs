use std::collections::HashMap;
use std::i32;
pub fn lenght(mut a2: i32) -> i32 {
    let mut len: i32 = 0;
    while a2 != 0 {
        len = len + 1;
        a2 = a2 / 10;
    }
    len
}
pub fn hashmap_english(a: i32) -> String {
    let mut text: HashMap<i32, &str> = HashMap::new();
    text.insert(0, "");
    text.insert(1, "One ");
    text.insert(2, "two ");
    text.insert(3, "three ");
    text.insert(4, "four ");
    text.insert(5, "five ");
    text.insert(6, "six ");
    text.insert(7, "seven ");
    text.insert(8, "eight ");
    text.insert(9, "nine ");
    text.insert(10, "ten ");
    text.insert(11, "eleven ");
    text.insert(12, "twelve ");
    text.insert(13, "thirteen ");
    text.insert(14, "fourteen ");
    text.insert(15, "fifteen ");
    text.insert(16, "sixteen ");
    text.insert(17, "seventeen ");
    text.insert(18, "eighteen ");
    text.insert(19, "nineteen ");
    text.insert(20, "twenty ");
    text.insert(30, "thirty ");
    text.insert(40, "fourty ");
    text.insert(50, "fifty ");
    text.insert(60, "sixty ");
    text.insert(70, "seventy ");
    text.insert(80, "eighty ");
    text.insert(90, "ninety ");
    let mut stri = String::new();
    stri.push_str(text.get(&a).unwrap());
    return stri;
}
pub fn hashmap_english2(a: i32) -> String {
    let mut output = String::new();
    let x: i32 = a % 100;
    if x > 0 && x <= 20 {
        output.push_str(&hashmap_english(x));
    } else {
        let y: i32 = x - (x % 10);
        let m: i32 = x % 10;
        output.push_str(&hashmap_english(y));
        output.push_str(&hashmap_english(m));
    }
    output
}
pub fn hashmap_english3(a: i32) -> String {
    let mut output = String::new();
    let mut len = lenght(a);

    while len >= 3 {
        if len > 9 {
            output.push_str("No out of range ");
            len = 0;
        }
        if len == 8 || len == 9 {
            output.push_str(&hashmap_english2(a / 10000000));
            if (a % 1000000000) / 100000000 != 0 || (a % 100000000) / 10000000 != 0 {
                output.push_str("crore ");
            }
            len = 7;
        }
        if len == 6 || len == 7 {
            output.push_str(&hashmap_english2(a / 100000));
            if (a % 10000000) / 1000000 != 0 || (a % 1000000) / 100000 != 0 {
                output.push_str("lakh ");
            }
            len = 5;
        }
        if len == 5 || len == 4 {
            output.push_str(&hashmap_english2(a / 1000));
            if (a % 100000) / 10000 != 0 || (a % 10000) / 1000 != 0 {
                output.push_str("thousand ");
            }
            len = 3;
        }
        if len == 3 {
            output.push_str(&hashmap_english2((a % 1000) / 100));
            if (a % 1000) / 100 != 0 {
                output.push_str("hundred ");
            }
            output.push_str(&hashmap_english2(a % 100));
            len = len - 1;
        }
    }
    output
}