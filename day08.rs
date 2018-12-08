use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate indextree;

struct NodeData {
    metadata: Vec<i32>
}

fn parse_input(path:&str) -> Vec<i32> {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut result = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                result = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            }
            Err(e) => println!("err: {}", e)
        }
    }
    result
}

fn build_tree_recursive(arena: &mut indextree::Arena<NodeData>, input: &Vec<i32>, cursor: &mut usize, parent: Option<indextree::NodeId> ) {
    if *cursor >= input.len() {
        return;
    }

    let child_count = input[*cursor];
    *cursor += 1;
    let meta_count = input[*cursor];
    *cursor += 1;

    let new_id = arena.new_node(NodeData {metadata:Vec::new()});
    if child_count > 0 {
        for _x in 0..child_count {
            build_tree_recursive(arena, input, cursor, Some(new_id));
        }
    }

    if parent.is_some() {
        assert!(parent.unwrap().append(new_id, arena).is_ok());
    }

    let node = arena.get_mut(new_id).unwrap();
    for _x in 0..meta_count {
        node.data.metadata.push(input[*cursor]);
        *cursor += 1;
    }
}

fn get_node_value(arena: &indextree::Arena<NodeData>, node_id: indextree::NodeId, val: &mut i32) {
    let child_count = node_id.children(arena).count();
    if child_count == 0 {
        let node= arena.get(node_id).unwrap();
        let meta_sum: i32 = node.data.metadata.iter().sum();
        *val += meta_sum;
    } else {
        let node= arena.get(node_id).unwrap();
        for x in &node.data.metadata {
            let index = *x as usize;
            if index > 0 && index <= child_count {
                let child_node = node_id.children(arena).nth(index - 1 as usize);
                if child_node.is_some() {
                    get_node_value(arena, child_node.unwrap(), val);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn part1(path:&str) -> i32 {
    use indextree::Arena;
    let arena = &mut Arena::new();
    let input = parse_input(path);

    let mut cursor: usize = 0;
    build_tree_recursive(arena, &input, &mut cursor, None);
    let mut sum: i32 = 0;
    for node in arena.iter() {
        let meta_sum: i32 = node.data.metadata.iter().sum();
        sum += meta_sum;
    }
    sum
}

#[allow(dead_code)]
fn part2(path:&str) -> i32 {
    use indextree::Arena;
    let arena = &mut Arena::new();
    let input = parse_input(path);

    let mut cursor: usize = 0;
    build_tree_recursive(arena, &input, &mut cursor, None);
    let mut val = 0;
    get_node_value(arena, indextree::NodeId::new(0), &mut val);
    val
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_example() {
        use part1;
        assert_eq!(part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\test.txt"), 138);
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt"), 43996);
    }

    #[test]
    fn test_part2_example() {
        use part2;
        assert_eq!(part2("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\test.txt"), 66);
    }

    #[test]
    fn test_part2_input() {
        use part2;
        assert_eq!(part2("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt"), 35189);
    }
}

fn main() {
}
