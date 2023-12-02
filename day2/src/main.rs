use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct BagGame {
    game: u8,
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let f = match read_file("data.txt") {
        Ok(v) => v,
        Err(e) => panic!("Error reading data file: {}", e),
    };

    let rules = BagGame {
        game: 0,
        red: 12,
        green: 13,
        blue: 14,
    };

    println!("Answers:");
    let t = tally_games(&f, rules);
    println!("Part 1: {}", t);
    let p = tally_game_power(f);
    println!("Part 2: {}", p);
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

fn tally_games(games: &Vec<String>, rules: BagGame) -> u64 {
    let mut tally: u64 = 0;
    for game in games.iter() {
        let g = parse_line(game);
        if possible_game(&g, &rules) {
            tally = tally + g.game as u64;
        }
    }
    tally
}

fn possible_game(g: &BagGame, rules: &BagGame) -> bool {
    if g.red > rules.red {
        return false;
    }

    if g.green > rules.green {
        return false;
    }

    if g.blue > rules.blue {
        return false;
    }

    return true;
}

fn convert_str(s: &str) -> u8 {
    let num: u8 = match s.trim().parse() {
        Ok(v) => v,
        Err(e) => panic!("Could not parse int from string: {}", e),
    };

    num
}

fn parse_line(s: &str) -> BagGame {
    let mut result: BagGame = BagGame {
        game: 0,
        red: 0,
        green: 0,
        blue: 0,
    };
    let g: Vec<&str> = s.split(":").collect();
    let str_id: Vec<&str> = g[0].split(" ").collect();
    result.game = convert_str(str_id[1]);

    let games: Vec<&str> = g[1].split(";").collect();
    for game in games.iter() {
        let pulls: Vec<&str> = game.split(",").collect();
        for pull in pulls.iter() {
            let detail: Vec<&str> = pull.trim().split(" ").collect();
            let n = convert_str(detail[0]);
            match detail[1].trim() {
                "red" => {
                    if result.red < n {
                        result.red = n
                    }
                }
                "green" => {
                    if result.green < n {
                        result.green = n
                    }
                }
                "blue" => {
                    if result.blue < n {
                        result.blue = n
                    }
                }
                _ => panic!("unexpected input!"),
            }
        }
    }

    result
}

fn tally_game_power(games: Vec<String>) -> u64 {
    let mut tally: u64 = 0;
    for game in games.iter() {
        let mut power: u64 = 1;
        let g = parse_line(game);
        power = power * g.red as u64;
        power = power * g.blue as u64;
        power = power * g.green as u64;
        tally = tally + power;
    }
    tally
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parse() {
        let i = convert_str("1");
        assert_eq!(i, 1);
    }

    #[test]
    fn test_large_num_parse() {
        let i = convert_str("178");
        assert_eq!(i, 178);
    }

    #[test]
    fn test_parse_with_space() {
        let i = convert_str(" 72");
        assert_eq!(i, 72);
    }

    #[test]
    fn test_game_is_possible() {
        let game = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game.game, 1);
        assert_eq!(game.red, 4);
        assert_eq!(game.green, 2);
        assert_eq!(game.blue, 6);
    }

    #[test]
    fn test_get_game_ids() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let rules = BagGame {
            game: 0,
            red: 12,
            green: 13,
            blue: 14,
        };
        let v: Vec<&str> = input.lines().collect();
        let vs = convert_vec_str_to_vec_string(v);
        let t = tally_games(&vs, rules);
        assert_eq!(t, 8);
    }

    #[test]
    fn test_game_power() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let v: Vec<&str> = input.lines().collect();
        let vs = convert_vec_str_to_vec_string(v);
        let t = tally_game_power(vs);
        assert_eq!(t, 2286);
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

