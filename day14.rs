#![feature(test)]

extern crate test;

#[allow(dead_code)]
fn part1(input: &str, to_create: i32) -> String {
    let mut scoreboard = Vec::new();
    for ch in input.chars() {
        scoreboard.push(ch.to_digit(10).unwrap());
    }
    let mut elfs: Vec<usize> = (0..scoreboard.len()).collect();
    let mut recipes_created = 0;
    loop {
        let sum: u32 = elfs.iter().fold(0, |sum, x| sum + scoreboard[*x]);
        let created = sum.to_string();
        for ch in created.chars() {
            recipes_created += 1;
            scoreboard.push(ch.to_digit(10).unwrap());
            if recipes_created >= to_create + 10 {
                let mut output = String::new();
                for x in 0..10 {
                    let ch2 = scoreboard[to_create as usize + x as usize].to_string().chars().nth(0).unwrap();
                    output.push(ch2);
                }
                return output;
            }
        }
        let scoreboard_len = scoreboard.len();
        for elf in elfs.iter_mut() {
            *elf = (*elf + scoreboard[*elf] as usize + 1) % scoreboard_len;
        }
    }
}

fn part2(input: &str, sequence: &str) -> u32 {
    let mut seq_vec = Vec::new();
    let mut scoreboard = Vec::new();
    for ch in input.chars() {
        scoreboard.push(ch.to_digit(10).unwrap());
    }
    for ch in sequence.chars() {
        seq_vec.push(ch.to_digit(10).unwrap());
    }
    let seq_len = seq_vec.len();
    let mut elfs: Vec<usize> = (0..scoreboard.len()).collect();
    loop {
        let sum: u32 = elfs.iter().fold(0, |sum, x| sum + scoreboard[*x]);
        let created = sum.to_string();
        for ch in created.chars() {
            scoreboard.push(ch.to_digit(10).unwrap());
            let len = scoreboard.len();
            if len >= seq_len {
                let mut found = true;
                for x in 0..seq_len {
                    let idx1 = len-x-1;
                    let idx2 = seq_len-x-1;
                    if scoreboard[idx1] != seq_vec[idx2] {
                        found = false;
                        break;
                    }
                }
                if found {
                    return len as u32 - seq_len as u32;
                }
            }
        }
        let scoreboard_len = scoreboard.len();
        for elf in elfs.iter_mut() {
            *elf = (*elf + scoreboard[*elf] as usize + 1) % scoreboard_len;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex0() {
        use part1;
        assert_eq!(part1("37", 9), "5158916779");
    }

    #[test]
    fn test_part1_ex1() {
        use part1;
        assert_eq!(part1("37", 5), "0124515891");
    }

    #[test]
    fn test_part1_ex2() {
        use part1;
        assert_eq!(part1("37", 18), "9251071085");
    }

    #[test]
    fn test_part1_ex3() {
        use part1;
        assert_eq!(part1("37", 2018), "5941429882");
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1("37", 513401), "5371393113");
    }

    #[test]
    fn test_part2_ex0() {
        use part2;
        assert_eq!(part2("37", "51589"), 9);
    }

    #[test]
    fn test_part2_ex1() {
        use part2;
        assert_eq!(part2("37", "01245"), 5);
    }

    #[test]
    fn test_part2_ex2() {
        use part2;
        assert_eq!(part2("37", "92510"), 18);
    }

    #[test]
    fn test_part2_ex3() {
        use part2;
        assert_eq!(part2("37", "59414"), 2018);
    }
}

fn main() {
    println!("result: {}", part2("37", "513401"));
}
