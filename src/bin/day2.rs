const INPUT: &str = include_str!("day2.txt");

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

#[allow(clippy::identity_op)]
fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("unexpected line {:?}", line),
        })
        .sum()
}

#[allow(clippy::identity_op)]
fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match line {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("unexpected line {:?}", line),
        })
        .sum()
}

#[cfg(test)]
#[test]
fn test() {
    const INPUT: &str = "\
A Y
B X
C Z";
    assert_eq!(part_1(INPUT), 15);
    assert_eq!(part_2(INPUT), 12);
}
