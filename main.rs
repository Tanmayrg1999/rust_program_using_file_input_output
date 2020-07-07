mod bengali;
mod english;
mod hindi;
mod gujrati;
mod marathi;
use std::i32;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::ffi::CString;
use std::io;
use std::str;
fn main() {
    let mut output = String::new();
    let mut file = File::open("C:/Users/tanma/CLionProjects/untitled/src/user_ip_file.txt")
        .expect("unable to open the  file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("unable to load data");
    let mut tot: i32 = 0;
    for line in content.lines() {
        tot = tot + 1;
    }
    let mut file = File::create("user_output.txt");
    let mut file1=OpenOptions::new().append(true).open("user_output.txt").expect("unable to open output file");
    let strings: Vec<&str> = content.split(",").collect(); // ["bananas", "apples", "pear"]
    let mut i = 1;
    for i in 1..tot + 1
    {
        let intamount: f64 = strings[(4 * i - 1) as usize]
            .trim()
            .parse()
            .ok()
            .expect("Program only process numbers,Enter number"); //converting string input to integer
        let line = intamount as i32; //integer currency
        let _diff: f64 = intamount - f64::from(line);
        let _fn: f64 = _diff * f64::from(100); //decimal value converted to integer by multiplying 100 to it
        let _int_fn = _fn.round();
        if line < 100 {
            output.push_str("INR ");
            output.push_str(&*english::hashmap_english2(line));
            if _int_fn != 0.0 {
                output.push_str(&*(_int_fn).to_string());
                output.push_str("/100,");
            } else {
                output.push_str(",");

            }
        } else {
            output.push_str("INR ");
            output.push_str(&*english::hashmap_english3(line));
            if _int_fn != 0.0 {
                output.push_str(&*(_int_fn).to_string());
                output.push_str("/100,");
            } else {
                output.push_str(",");
            }
        }
        let mut x=0;
        let mut y=i-1;
        for x in 0..4 {
                file1.write_all(strings[(4*y+x)as usize].as_bytes()).expect("write failed");
                file1.write_all(",".as_bytes()).expect("write failed");
        }
        file1.write_all(output.as_bytes()).expect("write failed");
        println!("file append success");
        output = "".to_string();
    }
    println!("{}",read_op());
    println!("UPDATE IP IS:");
    println!("{}",read_ip());
}
fn read_op()->String
{
    let mut output = String::new();
    let mut file = File::open("C:/Users/tanma/CLionProjects/untitled/user_output.txt")
        .expect("unable to open the  file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("unable to load data");
    let strings: Vec<&str> = content.split(",").collect();
    content.push_str("\r
    ++++\n");
    content
}
fn read_ip()->String
{
    let mut output = String::new();
    let mut file = File::open("C:/Users/tanma/CLionProjects/untitled/src/user_ip_file.txt")
        .expect("unable to open the  file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("unable to load data");
    let mut strings= String::new();
    let mut i=0;
    let mut tot1: i32 = 0;
    for line in content.lines() {
        tot1 = tot1 + 1;
    }
    let mut tot=0;
    let vector: Vec<&str> = content.split(",").collect(); // ["bananas", "apples", "pear"]
    for line in content.lines() {
       // strings.push_str(" ");
        strings.push_str(line);
      //  println!("{}",vector[(tot*4+3) as usize]);
        let intamount: f64 = vector[(tot*4+3) as usize]
            .trim()
            .parse()
            .ok()
            .expect("Program only process numbers,Enter number"); //converting string input to integer
        let amount = intamount as i32; //integer currency
        let _diff: f64 = intamount - f64::from(amount);
        let _fn: f64 = _diff * f64::from(100); //decimal value converted to integer by multiplying 100 to it
        let _int_fn = _fn.round(); // rounding the decimal equivalent
        if amount < 100 {
            strings.push_str("INR ");
            strings.push_str(&*english::hashmap_english2(amount));
            if _int_fn != 0.0 {
                strings.push_str(&*(_int_fn).to_string());
                strings.push_str("/100,\r\n");
            } else {
                strings.push_str(",\r\n");

            }
        } else {
            strings.push_str("INR ");
            strings.push_str(&*english::hashmap_english3(amount));
            if _int_fn != 0.0 {
                strings.push_str(&*(_int_fn).to_string());
                strings.push_str("/100,\r\n");
            } else {
                strings.push_str(",\r\n");
            }
        }
        tot=tot+1;
    }
    strings
}
    /*
    let mut _dec = String::new(); //asking user input in string format
    println!("Enter 1 for english 2 for hindi 3 for marathti 4 for bengali and 5 for gujrati");
    io::stdin()
        .read_line(&mut _dec)
        .expect("Failed to read line"); //user input
    let mydecision: i32 = _dec
        .trim()
        .parse()
        .ok()
        .expect("Program only process numbers,Enter number"); //converting string input to integer
    let _dec = mydecision as i32; //integer currency
    match mydecision {
        1 => {
            if amount < 100 {
                println!("{}",english::hashmap_english2(amount));
            } else {
                println!("{}",english:: hashmap_english3(amount));
            }
        }
        2 => println!("{}", hindi::hashmap_hindi2(amount)),
        3 => println!("{}", marathi::hashmap_marathi2(amount)),
        4 => println!("{}",bengali:: hashmap_bengali2(amount)),
        5 => println!("{}",gujrati:: hashmap_gujrati2(amount)),
        _ => println!("enter a valid input"),
    };
}*/
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_case1() {
        assert_eq!(english::hashmap_english2(98), "Ninety Eight ");
    }
    #[test]
    fn test_case2() {
        assert_eq!(english::hashmap_english2(9), "Nine ");
    }
    #[test]
    fn test_case3() {
        assert_eq!(english::hashmap_english3(316), "Three Hundred Sixteen ");
    }
    #[test]
    fn test_case4() {
        assert_eq!(
            english::hashmap_english3(3168),
            "Three Thousand One Hundred Sixty Eight "
        );
    }
    #[test]
    fn test_case5() {
        assert_eq!(
            english::hashmap_english3(31698),
            "Thirty One Thousand Six Hundred Ninety Eight "
        );
    }
    #[test]
    fn test_case6() {
        assert_eq!(english::hashmap_english3(100000), "One Lakh ");
    }
    #[test]
    fn test_case7() {
        assert_eq!(
            english::hashmap_english3(1023456),
            "Ten Lakh Twenty Three Thousand Four Hundred Fifty Six "
        );
    }
    #[test]
    fn test_case8() {
        assert_eq!(english::hashmap_english3(10000000), "One Crore ");
    }
    #[test]
    fn test_case9() {
        assert_eq!(english::hashmap_english3(100000000), "Ten Crore ");
    }
    #[test]
    fn test_case10() {
        assert_eq!(english::hashmap_english3(1100000000), "No out of range ");
    }
    #[test]
    fn test_case11() {
        assert_eq!(marathi::hashmap_marathi2(98), "अठ्ठ्याण्णव");
    }
    #[test]
    fn test_case12() {
        assert_eq!(marathi::hashmap_marathi2(9), "नऊ ");
    }
    #[test]
    fn test_case13() {
        assert_eq!(marathi::hashmap_marathi2(316), "तीन शे सोळा ");
    }
    #[test]
    fn test_case14() {
        assert_eq!(
            marathi::hashmap_marathi2(3168),
            "तीन हजार एक शे अडुसष्ठ "
        );
    }
    #[test]
    fn test_case15() {
        assert_eq!(
            marathi::hashmap_marathi2(31698),
            "एकतीस हजार सहा शे अठ्ठ्याण्णव"
        );
    }
    #[test]
    fn test_case16() {
        assert_eq!(marathi::hashmap_marathi2(100000), "एक लाख ");
    }
    #[test]
    fn test_case17() {
        assert_eq!(
            marathi::hashmap_marathi2(1023456),
            "दहा लाख तेवीस हजार चार शे छप्पन्न "
        );
    }
    #[test]
    fn test_case18() {
        assert_eq!(marathi::hashmap_marathi2(10000000), "एक कोटी ");
    }
    #[test]
    fn test_case19() {
        assert_eq!(marathi::hashmap_marathi2(100000000), "दहा कोटी ");
    }
    #[test]
    fn test_case20() {
        assert_eq!(marathi::hashmap_marathi2(1100000000), "No out of range ");
    }
    #[test]
    fn test_case21() {
        assert_eq!(bengali::hashmap_bengali2(1), "এক ");
    }#[test]
    fn test_case22() {
        assert_eq!(bengali::hashmap_bengali2(45), "পঁয়তাল্লিশ ");
    }#[test]
    fn test_case23() {
        assert_eq!(bengali::hashmap_bengali2(456), "চার শো ছাপ্পান্নো ");
    }#[test]
    fn test_case24() {
        assert_eq!(bengali::hashmap_bengali2(4567), "চার হাজার পাঁচ শো সাতষট্টি ");
    }#[test]
    fn test_case25() {
        assert_eq!(bengali::hashmap_bengali2(45678), "পঁয়তাল্লিশ হাজার ছয় শো আটাত্তর ");
    }#[test]
    fn test_case26() {
        assert_eq!(bengali::hashmap_bengali2(456789), "চার লক্ষ  ছাপ্পান্নো হাজার সাত শো ঊনোনব্বই ");
    }#[test]
    fn test_case27() {
        assert_eq!(bengali::hashmap_bengali2(4567890), "পঁয়তাল্লিশ লক্ষ  সাতষট্টি হাজার আট শো নব্বুই ");
    }#[test]
    fn test_case28() {
        assert_eq!(bengali::hashmap_bengali2(45678901), "চার কোটি ছাপ্পান্নো লক্ষ  আটাত্তর হাজার নয় শো এক ");
    }#[test]
    fn test_case29() {
        assert_eq!(bengali::hashmap_bengali2(456789012), "পঁয়তাল্লিশ কোটি সাতষট্টি লক্ষ  ঊনোনব্বই হাজার বারো ");
    }#[test]
    fn test_case30() {
        assert_eq!(bengali::hashmap_bengali2(1100000000), "No out of range ");
    }#[test]
    fn test_case31() {
        assert_eq!(hindi::hashmap_hindi2(6), "छह ");
    }#[test]
    fn test_case32() {
        assert_eq!(hindi::hashmap_hindi2(67), "सड़सठ ");
    }#[test]
    fn test_case33() {
        assert_eq!(hindi::hashmap_hindi2(678), "छह सौ अठहतर ");
    }#[test]
    fn test_case34() {
        assert_eq!(hindi::hashmap_hindi2(6789), "छह हजार सात सौ नवासी ");
    }#[test]
    fn test_case35() {
        assert_eq!(hindi::hashmap_hindi2(67890), "सड़सठ हजार आठ  सौ नब्बे ");
    }#[test]
    fn test_case36() {
        assert_eq!(hindi::hashmap_hindi2(678901), "छह लाख अठहतर हजार नौ सौ एक ");
    }#[test]
    fn test_case37() {
        assert_eq!(hindi::hashmap_hindi2(6789012), "सड़सठ लाख नवासी हजार बारह ");
    }#[test]
    fn test_case38() {
        assert_eq!(hindi::hashmap_hindi2(67890123), "छह करोड़ अठहतर लाख नब्बे हजार एक सौ तेइस ");
    }#[test]
    fn test_case39() {
        assert_eq!(hindi::hashmap_hindi2(678901234), "सड़सठ करोड़ नवासी लाख एक हजार दो सौ चौंतीस ");
    }#[test]
    fn test_case40() {
        assert_eq!(hindi::hashmap_hindi2(1100000000), "No out of range ");
    }#[test]
    fn test_case41() {
        assert_eq!(gujrati::hashmap_gujrati2(2), "બે  ");
    }#[test]
    fn test_case42() {
        assert_eq!(gujrati::hashmap_gujrati2(23), "તેવીસ ");
    }
    #[test]
    fn test_case43() {
        assert_eq!(gujrati::hashmap_gujrati2(234), "બે  સો ચોત્રીસ ");
    }#[test]
    fn test_case44() {
        assert_eq!(gujrati::hashmap_gujrati2(2345), "બે  હજાર ત્રણ સો પિસ્તાલીસ ");
    }#[test]
    fn test_case45() {
        assert_eq!(gujrati::hashmap_gujrati2(23456), "તેવીસ હજાર ચાર સો છપ્પન ");
    }#[test]
    fn test_case46() {
        assert_eq!(gujrati::hashmap_gujrati2(234567), "બે  લાખ ચોત્રીસ હજાર પાંચ સો સડસઠ ");
    }#[test]
    fn test_case47() {
        assert_eq!(gujrati::hashmap_gujrati2(2345678), "તેવીસ લાખ પિસ્તાલીસ હજાર છ સો ઇઠ્યોતેર ");
    }#[test]
    fn test_case48() {
        assert_eq!(gujrati::hashmap_gujrati2(23456789), "બે  કરોડ઼ ચોત્રીસ લાખ છપ્પન હજાર સાત સો નેવ્યાસી ");
    }#[test]
    fn test_case49() {
        assert_eq!(gujrati::hashmap_gujrati2(234567890),"તેવીસ કરોડ઼ પિસ્તાલીસ લાખ સડસઠ હજાર આઠ  સો નેવું ");
    }#[test]
    fn test_case50() {
        assert_eq!(gujrati::hashmap_gujrati2(1100000000), "No out of range ");
    }
    #[test]
    fn test_case51() {
        assert_eq!(read_ip(),read_op());
    }
}
