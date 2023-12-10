use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

//A node in the graph
struct Node {
    val: String,
    left: String,
    right: String,
}

fn main() -> Result<(), std::io::Error> {
    let (directions, nodes) = read_file("data.txt")?;
    let c = walk(&directions, &nodes);
    println!("p1: {}", c);
    let n = bfs(nodes);
    println!("p2: {}", n);
    Ok(())
}

fn walk(steps: &String, nodes: &Vec<Node>) -> usize {
    let mut node = match nodes.iter().find(|x| x.val == "AAA") {
        Some(val) => val,
        None => panic!("could not find starting value!")
    };
    let mut set = 0;
    loop {
        for (i,d) in steps.chars().enumerate() {
            if node.val == "ZZZ" {
                return set + i
            }
            let next_val = match d {
                'L' => &node.left,
                _ => &node.right
            };
            node = match nodes.iter().find(|x| x.val == *next_val) {
                Some(val) => val,
                None => panic!("could not find next value: {}", next_val)
            };
        }
        set = set + steps.len();
    }
}

fn bfs(nodes: Vec<Node>) -> i16 {
    let mut seen: Vec<&String> = Vec::new();
    let start = match nodes.iter().position(|x| x.val == "AAA") {
        Some(val) => val,
        None => panic!("could not find starting value!")
    };
    let goal = "ZZZ".to_string();
    seek(&nodes[start], &nodes, &mut seen, &goal, 0)
}

fn seek<'a>(vertex: &'a Node, nodes: &'a Vec<Node>, visited: &mut Vec<&'a String>, target: &String, depth: i16) -> i16 {
    if vertex.val == *target {
        return depth
    }
    for v in visited.iter() {
        if **v == vertex.val {
            return -1
        }
    }
    visited.push(&vertex.val);
    let lft = match nodes.iter().find(|n| n.val == vertex.left) {
        Some(v) => v,
        None => panic!("could not find vertex {}!", vertex.left)
    };
    let ldepth = seek(lft, nodes, visited, target, depth +1);
    let rht = match nodes.iter().find(|n| n.val == vertex.right) {
        Some(v) => v,
        None => panic!("could not find vertex {}!", vertex.right)
    };
    let rdepth = seek(rht, nodes, visited, target, depth +1);
    if rdepth > ldepth {
        return rdepth
    }
    return ldepth
}

fn read_file(file_name: &str) -> Result<(String, Vec<Node>), std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    //let mut lines = reader.lines();
    let mut graph: Vec<Node> = Vec::new();
    let mut directions: String = "".to_string();
    for (i, ln) in reader.lines().enumerate() {
        match i {
            0 => {
                directions = ln.unwrap(); //panic if needed bby
            },
            1 => continue,
            _ => {
                match ln {
                    Ok(v) => {
                        graph.push(Node {
                            val: v[..3].to_string(),
                            left: v[7..10].to_string(),
                            right: v[12..15].to_string(),
                        });
                    }
                    Err(e) => return Err(e)
                }
            }

        }
    }


    Ok((directions, graph))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting_wins() {}

    #[test]
    fn test_parsing_nums() {
        let input: Vec<&str> = r#"RLRRR

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#
            .lines()
            .collect();
        let s = convert_vec_str_to_vec_string(input);
        assert_eq!(s.len(), 2);
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
