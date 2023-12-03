use std::str;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct PartNum {
    line: usize,
    pos: usize,
    val: String,
    nval: usize,
}

    
#[allow(dead_code)]
struct SpecialChar {
    line: usize,
    pos: usize,
    val: String,
}

fn main() -> Result<(), std::io::Error>{
    let data = read_file("data.txt")?;
    let (nums, symbols) = tokenize(data);
    let ans1 = get_part_count(&nums, &symbols);
    println!("Part 1 answer: {}", ans1);

    Ok(())
}

fn get_part_count(nums: &Vec<PartNum>, symbols: &Vec<SpecialChar>) -> usize {
    let part_nums = nums.iter().filter(|n| check_if_part_num(n, &symbols)).collect::<Vec<_>>();
    let part_1 = part_nums.iter().fold(0, |acc, p| acc + p.nval);
    part_1
}

fn check_if_part_num(part: &PartNum, symbols: &Vec<SpecialChar>) -> bool {
    symbols.iter().any(|s| matching_symbol(part, s))
}

fn matching_symbol(p: &PartNum, s: &SpecialChar) -> bool {
    if s.line < max(p.line, 1) - 1 || s.line > (p.line + 1) {
        return false;
    }
    if s.pos < max(p.pos, 1) - 1 || s.pos > (p.pos + p.val.len()) {
        return false;
    }
    true
}

fn tokenize(input: Vec<String>) -> (Vec<PartNum>, Vec<SpecialChar>) {
    let mut numbers: Vec<PartNum> = Vec::new();
    let mut symbols: Vec<SpecialChar> = Vec::new();
    for (n, line) in input.iter().enumerate() {
        let mut pos: usize = 0;
        let charz: Vec<char> = line.chars().collect();
        while pos < line.len() {
            //should I use a match statement? Can I use contineut within a match?
            if charz[pos] == '.' {
                pos = pos + 1;
                continue;
            }
            if charz[pos].is_digit(10) {
                let mut end = pos + 1;
                while end < line.len() && line.chars().nth(end).unwrap().is_digit(10) {
                    end += 1;
                }
                let nval = match line[pos..end].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => panic!("unable to parse u16 from string! {}", e)
                };
                numbers.push(PartNum{
                    pos,
                    nval,
                    line: n,
                    val: line[pos..end].to_string(),
                });
                pos = end;
                continue;
            }

            //TODO: Assumption is it's a . digit or symbol. Could be wrong
            symbols.push(SpecialChar{
                line: n,
                pos,
                val: charz[pos].to_string(),
            });
            pos = pos + 1;
        }
    }

    (numbers, symbols)
}

fn read_file(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut v: Vec<String> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        v.push(line);
    }

    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_numbers() {
        let s = r#"467..114..
        ...*..4..."#;
        let my_str: Vec<&str> = s.lines().collect();
        let input = convert_vec_str_to_vec_string(my_str);
        let (ans, _) = tokenize(input);
        //ensure we have the right number of solutions to view
        assert_eq!(ans.len(), 3);
        assert_eq!(ans[0].nval, 467);

        assert_eq!(ans[1].nval, 114);
        assert_eq!(ans[1].line, 0);

        assert_eq!(ans[2].nval, 4);
        assert_eq!(ans[2].line, 1);

    }

    #[test]
    fn test_tokenizer_symbol() {
        let s: Vec<&str> = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#.lines().collect();
        let input = convert_vec_str_to_vec_string(s);
        let (_, ans) = tokenize(input);
        assert_eq!(ans.len(), 6);
        assert_eq!(ans[0].val, "*");
        assert_eq!(ans[2].line, 4);
        assert_eq!(ans[2].pos, 3);
        assert_eq!(ans[2].val, "*");
    }

    #[test]
    fn test_part_one() {
        let s: Vec<&str> = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#.lines().collect();
        let input = convert_vec_str_to_vec_string(s);
        let (n, s) = tokenize(input);
        let ans = get_part_count(&n, &s);
        assert_eq!(ans, 4361);
    }

    fn convert_vec_str_to_vec_string(vec_str: Vec<&str>) -> Vec<String> {
        let mut vec_string = Vec::new();
        for s in vec_str {
            match s {
                s => vec_string.push(s.to_owned()),
            }
        }
        vec_string
    }
}
