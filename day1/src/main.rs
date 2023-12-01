use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

//Notes, this is correct but we need to decouple IO from 
//processing. This input isn't big so we could do the entire thing in memory
//Work on steaming it later, if it was we'd just pass it off to some channel
fn main() -> Result<(), std::io::Error> {
    let lines = read_file("data.txt")?;
    let ans = process_lines(lines)?;
    print!("The total number is: {}\n", ans);
    Ok(())
}

fn process_lines(lines: Vec<Vec<u8>>) -> Result<usize, std::io::Error> {
    let mut total: usize = 0;
    for line in lines.iter() {
        let c = get_calibration(line);
        total = total + c as usize;
    }

    Ok(total)
}

fn read_file(file_name: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut v: Vec<Vec<u8>> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        v.push(line.into_bytes());
    }

    Ok(v)
}

pub fn get_calibration(data: &[u8]) -> u8 {
    let condition = |b| (b >= b'0') && (b <= b'9');

    let f = match data.iter().find(|&byte| condition(*byte)) {
        Some(v) => v, 
        None => return 0
    };
    let b = match data.iter().rev().find(|&byte| condition(*byte)) {
        Some(v) => v,
        None => return 0
    };

    let byte_val: &[u8] = &[*f, *b];
    let str_val = match str::from_utf8(byte_val) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
    };

    let number = match u8::from_str_radix(str_val, 10) {
        Ok(v) => v,
        Err(e) => panic!("Invalid Character is not a u8: {}", e)
    };
    // let number: u8 = [&f, &b].iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as u8);
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex0() {
        let o = get_calibration("1abc2".as_bytes());
        assert_eq!(o, 12);
    }

    #[test]
    fn test_ex1() {
        let o = get_calibration("pqr3stu8vwx".as_bytes());
        assert_eq!(o, 38);
    }

    #[test]
    fn test_ex2() {
        let o = get_calibration("a1b2c3d4e5f".as_bytes());
        assert_eq!(o, 15);
    }

    #[test]
    fn test_one_digit() {
        let o = get_calibration("treb7uchet".as_bytes());
        assert_eq!(o, 77);
    }

    #[test]
    fn test_no_digits() {
        let o = get_calibration("trebuchet".as_bytes());
        assert_eq!(o, 0);
    }

    #[test]
    fn test_processing_multiple_lines() {
        let datar = vec![
            "1abc2".as_bytes().to_vec(),
            "pqr3stu8vwx".as_bytes().to_vec(),
            "a1b2c3d4e5f".as_bytes().to_vec(),
            "treb7uchet".as_bytes().to_vec(),
        ];
        let total = match process_lines(datar) {
            Ok(v) => v,
            Err(e) => panic!("could not process lines in test! {}", e)
        };
        assert_eq!(total, 142);
    }
}
