#![feature(test)]

use std::collections::VecDeque;

extern crate test;

fn solution(num_players:usize, last_marble:u32) -> u32 {
    let mut circle = VecDeque::with_capacity(last_marble as usize);
    let mut scores = vec![0u32; num_players];
    circle.push_back(0u32);
    for marble in 1..last_marble+1 {
        if marble % 23 == 0 {
            let player = marble as usize % num_players;
            scores[player] += marble;
            for _ in 0..7 {
                let tmp = circle.pop_back().unwrap();
                circle.push_front(tmp);
            }
            scores[player] += circle.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let tmp = circle.pop_front().unwrap();
                circle.push_back(tmp);
            }
            circle.push_front(marble);
        }
    }
    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    //use test::Bencher;

    #[test]
    fn test_part1_ex_0() {
        use solution;
        assert_eq!(solution(9, 25), 32);
    }

    #[test]
    fn test_part1_ex_1() {
        use solution;
        assert_eq!(solution(10, 1618), 8317);
    }

    #[test]
    fn test_part1_ex_2() {
        use solution;
        assert_eq!(solution(13, 7999), 146373);
    }

    #[test]
    fn test_part1_ex_3() {
        use solution;
        assert_eq!(solution(17, 1104), 2764);
    }

    #[test]
    fn test_part1_ex_4() {
        use solution;
        assert_eq!(solution(21, 6111), 54718);
    }

    #[test]
    fn test_part1_ex_5() {
        use solution;
        assert_eq!(solution(30, 5807), 37305);
    }

    #[test]
    fn test_part1_input() {
        use solution;
        assert_eq!(solution(486, 70833), 373597);
    }

    #[test]
    fn test_part2_input() {
        use solution;
        assert_eq!(solution(486, 70833 * 100), 2954067253);
    }
}

fn main() {
}
