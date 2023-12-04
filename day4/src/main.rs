use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error>{
    let data = read_file("data.txt")?;
    let result = data.iter().fold(0, |acc, game| acc + score_game(&game));
    print!("Part 1: {}\n", result);

    let ans2 = count_tickets(&data);
    print!("Part 2: {}\n", ans2);

    Ok(())
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

fn count_tickets(data: &Vec<String>) -> u32 {
    let mut count: u32 = data.len() as u32;
    for i in 0..data.len() {
        count = count + score_game_part2(data, i);
    }
    count
}

fn score_game_part2(dataset: &Vec<String>, idx: usize) -> u32 {
    let input = &dataset[idx];
    let mut score = 0;
    let mut pos = 0;
    let mut winning_numbers: Vec<&str> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if c == ':' {
            //ASSUMPTION: always a space after the semi-colon,
            pos = i + 2;
            break;
        }
    }
    let charz: Vec<char> = input.chars().collect();
    while pos < input.len() {
        if charz[pos] == '|' {
            pos = pos + 2;
            break;
        }
        let n: &str = &input[pos..(pos+2)];
        winning_numbers.push(n);
        //ASSUMPTION: no winning number is above 99
        pos = pos + 3;
    }

    while pos < input.len() {
        let n: &str = &input[pos..(pos+2)];
        for val in winning_numbers.iter() {
            if **val == *n {
                score = score + 1;
                break;
            }
        }
        
        pos = pos + 3;
    }
    if score > 0 {
        for k in 1..=score as usize {
            score = score + score_game_part2(dataset, idx + k);
        }
    }
    score
}

fn score_game(input: &String) -> u32 {
    let mut score = 0;
    let mut pos = 0;
    let mut winning_numbers: Vec<&str> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if c == ':' {
            //ASSUMPTION: always a space after the semi-colon,
            pos = i + 2;
            break;
        }
    }
    let charz: Vec<char> = input.chars().collect();
    while pos < input.len() {
        if charz[pos] == '|' {
            pos = pos + 2;
            break;
        }
        let n: &str = &input[pos..(pos+2)];
        winning_numbers.push(n);
        //ASSUMPTION: no winning number is above 99
        pos = pos + 3;
    }

    while pos < input.len() {
        let n: &str = &input[pos..(pos+2)];
        for val in winning_numbers.iter() {
            if **val == *n {
                if score == 0 {
                    score = 1;
                } else {
                    score = 2 * score;
                }
                break;
            }
        }
        
        pos = pos + 3;
    }
    score
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_all_games() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#.lines().collect();
        let data = convert_vec_str_to_vec_string(input);
        let part_1 = data.iter().fold(0, |acc, game| acc + score_game(&game));
        assert_eq!(part_1, 13);
    }

    #[test]
    fn test_game() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#.lines().collect();
        let s = convert_vec_str_to_vec_string(input);
        let score = score_game(&s[0]);
        assert_eq!(score, 8);
    }

    #[test]
    fn test_counting_tickets() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#.lines().collect();
        let data = convert_vec_str_to_vec_string(input);
        let ans = count_tickets(&data);
        assert_eq!(ans, 30);
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
