use std::collections::BinaryHeap;
use std::str::FromStr;

const INPUT: &str = include_str!("day1.txt");
#[cfg(test)]
const TEST_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

fn main() {
    let (part1, part2) = answers(INPUT);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn answers(input: &str) -> (u32, u32) {
    let mut heap = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split_whitespace()
                .map(|s| u32::from_str(s).unwrap())
                .sum::<u32>()
        })
        .collect::<BinaryHeap<_>>();
    let part1 = heap.pop().unwrap();
    let part2 = part1 + heap.pop().unwrap() + heap.pop().unwrap();
    (part1, part2)
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(answers(TEST_INPUT), (24000, 45000));
}
