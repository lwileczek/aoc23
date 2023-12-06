use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error>{
    let data = read_file("data.txt")?;

    let ans1 = p1(&data);
    println!("Part 1: {}", ans1);

    let ans2 = p2(&data);
    println!("Part 2: {}", ans2);

    Ok(())
}

fn p1 (data: &Vec<String>) -> u64 {
    let times = parse_nums(&data[0]);
    let distances = parse_nums(&data[1]);
    if times.len() != distances.len() {
        println!("time: {} | dist: {}", times.len(), distances.len());
        panic!("didn't match up times and distences correctly")
    }
    let mut ans: u64 = 1;
    for (t, d) in times.iter().zip(distances.iter()) {
        let wins = count_wins(t, d);
        ans = ans * wins;
    }
    ans
}

fn p2 (data: &Vec<String>) -> u64 {
    let t = parse_big_num(&data[0]);
    let d = parse_big_num(&data[1]);
    count_wins(&t, &d)
}

//here the problem says the kerning is off and the line is supposed to be one big number
//so we find all the digits and combine them.
fn parse_big_num(s: &String) -> u64 {
    let mut charz: Vec<char> = Vec::new();
    for c in s.chars() {
        if c.is_digit(10) {
            charz.push(c);
        }
    }
    let str_num: String = charz.iter().collect();
    let num = match str_num.parse::<u64>() {
        Ok(v) => v,
        Err(e) => panic!("could not parse big string into u64: {}", e)
    };

    num
}

//find the first place where we win the game, and use the fact that the results
//are symmetric to count the total number of ways we can win
fn count_wins(t: &u64, d: &u64) -> u64 {
    let mut eclipse = 0; 
    let half = t / 2;
    for k in 1..half {
        if project_distance(k, &t) > *d {
            eclipse = k;
            break;
        }
    }
    if t % 2 == 0 {
        return (half-eclipse)*2 + 1;
    }
    (half-eclipse + 1)*2
}

//No required at all, pretending the inputs can be funky and trying to write
//something cleaner with a simple check.
fn project_distance(t: u64, total: &u64) -> u64 {
    if *total <= t {
        return 0;
    }
    t * (total - t)
}

//read through the line character by character grabbing the multi-digit numbers
//this is pretty much how I've been parsing numbers out each day.
//I'm not sure what the best practice is
fn parse_nums(line: &String) -> Vec<u64>{
    let charz: Vec<char> = line.chars().collect();
    let mut ans: Vec<u64> = Vec::new();
    let mut pos = 0;
    while pos < line.len() {
        if charz[pos].is_digit(10) {
            let mut end = pos + 1;
            while end < line.len() && line.chars().nth(end).unwrap().is_digit(10) {
                end += 1;
            }
            let n = match line[pos..end].parse::<u64>() {
                Ok(v) => v,
                Err(e) => panic!("unable to parse u64 from string! {}", e)
            };

            ans.push(n);
            pos = end;
            continue
        }
        pos = pos + 1
    }
    ans
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
    fn test_getting_times() {
        let input=r#"Time:      7  15   30
Distance:  9  40  200"#.lines().collect();
        let data = convert_vec_str_to_vec_string(input);
        let n = p1(&data);
        assert_eq!(n, 288);
    }

    #[test]
    fn test_kerning() {
        let input=r#"Time:      7  15   30
Distance:  9  40  200"#.lines().collect();
        let data = convert_vec_str_to_vec_string(input);
        let n = p2(&data);
        assert_eq!(n, 71503);
    }

    #[test]
    fn test_projecting_distance() {
        let data = vec![
            vec![0, 0],
            vec![1, 6],
            vec![2, 10],
            vec![3, 12],
            vec![4, 12],
            vec![5, 10],
            vec![6, 6],
            vec![7, 0],
        ];
         let total: u64 = 7;
        for input in data.iter() {
            let n =  project_distance(input[0], &total);
            assert_eq!(n, input[1]);
        }
    }

    #[test]
    fn test_counting_wins() {
        let data = vec![
            vec![7, 9, 4],
            vec![15, 40, 8],
            vec![30, 200, 9],
        ];
        for input in data.iter() {
            let n =  count_wins(&input[0], &input[1]);
            assert_eq!(n, input[2]);
        }
    }

    #[test]
    fn test_parsing_nums() {
        let s = "Time:      7  15   30";
        let nums = parse_nums(&s.to_string());
        assert_eq!(nums.len(), 3);

        assert_eq!(nums[0], 7);
        assert_eq!(nums[1], 15);
        assert_eq!(nums[2], 30);
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
