use std::str;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
enum HandType {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug,Eq, Ord, PartialOrd, PartialEq)]
struct Hand {
    //rank it in poker
    htype: HandType,
    //the actual hand for reference
    hand: String,
    //the bid value
    bid: u16,
}

fn main() -> Result<(), std::io::Error>{
    let data = read_file("data.txt")?;

    let ans1 = p1(&data);
    println!("Part 1: {}", ans1);

    Ok(())
}

fn p1 (data: &Vec<String>) -> usize {
    //TODO: .sort().fold()
    let mut hands: Vec<Hand> =  data.iter().map(parse_hand).collect();
    //hands.sort_unstable_by_key(|item| (item.htype, item.hand.clone()));
    sort(&mut hands);
    hands.iter().enumerate().fold(0, |acc, (idx, h)| acc + (idx+1)*h.bid as usize)
}

fn sort(items: &mut [Hand]) {
    items.sort_unstable_by(|a, b| {
        match a.htype.cmp(&b.htype) {
            //Ordering::Equal => { a.column.cmp(&b.column) }
            Ordering::Equal => {
                let mut o: Ordering = Ordering::Equal;
                for (ca, cb) in a.hand.chars().zip(b.hand.chars()) {
                    if ca == cb {
                        continue
                    }
                    let va = value_card(ca);
                    let vb = value_card(cb);
                    o = va.cmp(&vb);
                    break
                }
                o
            }
            v => { v }
        }
    });
}

fn value_card(c: char) -> u8 {
        match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            _ => panic!("UNKNOWN CARD!")
        }
}

fn parse_hand(line: &String) -> Hand {
    let space = match line.chars().position(|c| c == ' ') {
        Some(n) => n,
        None => panic!("couldn't find the break point in this line!")
    };

    let bid = match line[(space+1)..].parse::<u16>() {
        Ok(v) => v,
        Err(e) => panic!("could not parse the bid! {}", e)
    };

    Hand{
        htype: categorize_hand(&line[..space].to_string()),
        bid,
        hand: line[..space].to_string()
    }
}

fn categorize_hand(h: &String) -> HandType {
    let mut cards: Vec<u8> = vec![0; 13];
    for c in h.chars() {
        match c {
            'A' => cards[12] = cards[12] + 1,
            'K' => cards[11] = cards[11] + 1,
            'Q' => cards[10] = cards[10] + 1,
            'J' => cards[9] = cards[9] + 1,
            'T' => cards[8] = cards[8] + 1,
            '9' => cards[7] = cards[7] + 1,
            '8' => cards[6] = cards[6] + 1,
            '7' => cards[5] = cards[5] + 1,
            '6' => cards[4] = cards[4] + 1,
            '5' => cards[3] = cards[3] + 1,
            '4' => cards[2] = cards[2] + 1,
            '3' => cards[1] = cards[1] + 1,
            '2' => cards[0] = cards[0] + 1,
            _ => panic!("UNKNOWN CARD!")
        }
    }
    let result = cards.iter()
        .filter(|x| **x != 0)
        .fold(1, |acc, x| acc*x);

    match result {
        6 => HandType::Full,
        5 => HandType::Five,
        4 => {
            match cards.iter().filter(|x| **x == 2).count() {
                2 => HandType::Two,
                0 => HandType::Four,
                _ => panic!("unexpected result of 4")
            }
        },
        3 => HandType::Three,
        2 => HandType::One,
        1 => HandType::High,
        _ => panic!("unexpected hand count!")
    }
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
    struct HandResult {
        h: String,
        t: HandType,
    }

    struct ParseHandAnswer {
        s: String,
        h: Hand,
    }

    #[test]
    fn test_categorizing_hands() {
        let hands = vec![
            HandResult{h: "32T3K".to_string(), t: HandType::One},
            HandResult{h: "KK677".to_string(), t: HandType::Two},
            HandResult{h: "KTJJT".to_string(), t: HandType::Two},
            HandResult{h: "T55J5".to_string(), t: HandType::Three},
            HandResult{h: "QQQJA".to_string(), t: HandType::Three},
            HandResult{h: "QQQJQ".to_string(), t: HandType::Four},
            HandResult{h: "QQQQQ".to_string(), t: HandType::Five},
            HandResult{h: "QJQJQ".to_string(), t: HandType::Full},
            HandResult{h: "32T4K".to_string(), t: HandType::High},
        ];

        for hand in hands.iter() {
            let ct = categorize_hand(&hand.h);
            assert_eq!(ct, hand.t);
        }
    }

    #[test]
    fn test_parse_hand() {
        let hands = vec![
            ParseHandAnswer{s: "32T3K 765".to_string(), h: Hand{htype: HandType::One, bid: 765, hand: "32T3K".to_string()}},
            ParseHandAnswer{s: "KK677 684".to_string(), h: Hand{htype: HandType::Two, bid: 684, hand: "KK677".to_string()}},
            ParseHandAnswer{s: "KTJJT 28".to_string(), h: Hand{htype: HandType::Two, bid: 28, hand: "KTJJT".to_string()}},
            ParseHandAnswer{s: "T55J5 220".to_string(), h: Hand{htype: HandType::Three, bid: 220, hand: "T55J5".to_string()}},
            ParseHandAnswer{s: "QQQJA 483".to_string(), h: Hand{htype: HandType::Three, bid: 483, hand: "QQQJA".to_string()}},
        ];

        for hand in hands.iter() {
            let h = parse_hand(&hand.s);
            assert_eq!(h.htype, hand.h.htype);
            assert_eq!(h.bid, hand.h.bid);
            assert_eq!(h.hand, hand.h.hand);
        }
    }

    #[test]
    fn test_sorting_hand() {
        let mut hands = vec![
            Hand{htype: HandType::Three, bid: 220, hand: "T55J5".to_string()},
            Hand{htype: HandType::Two, bid: 684, hand: "KK677".to_string()},
            Hand{htype: HandType::One, bid: 765, hand: "32T3K".to_string()},
            Hand{htype: HandType::Two, bid: 28, hand: "KTJJT".to_string()},
            Hand{htype: HandType::Three, bid: 483, hand: "QQQJA".to_string()},
        ];
        let solution = vec![
            "32T3K".to_string(),
            "KTJJT".to_string(),
            "KK677".to_string(),
            "T55J5".to_string(),
            "QQQJA".to_string(),
        ];

        sort(&mut hands);
        for (h, s) in hands.iter().zip(solution.iter()) {
            assert_eq!(h.hand, *s);
        }
    }

    #[test]
    fn test_p1() {
        let input=r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#.lines().collect();
        let data = convert_vec_str_to_vec_string(input);
        let n = p1(&data);
        assert_eq!(n, 6440);
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
