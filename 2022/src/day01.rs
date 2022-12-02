fn main() {
    let input = include_str!("../input/day01.input");

    // Part one
    let max = first_puzzle(input);
    println!("The elf carrying the most calories has {} calories.", max);

    // Part two
    let top_three_sum = second_puzzle(input);
    println!("The top three elves are carrying {} calories in total.", top_three_sum);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day01.input");
    
    assert_eq!(first_puzzle(sample), 24000);
    assert_eq!(second_puzzle(sample), 45000);
}

fn first_puzzle(source: &str) -> u64 {
    source
        .split("\n\n")
        .map(|line|
            line
                .lines()
                .map(to_u64)
                .sum()
        )
        .max()
        .unwrap()
}

fn second_puzzle(source: &str) -> u64 {
    let mut top = [0, 0, 0];

    source
        .split("\n\n")
        .map(|line|
            line
                .lines()
                .map(to_u64)
                .sum()
        )
        .for_each(|count| {
            if count > top[0] {
                top[2] = top[1];
                top[1] = top[0];
                top[0] = count;
            } else if count > top[1] {
                top[2] = top[1];
                top[1] = count
            } else if count > top[2] {
                top[2] = count
            }
        });

    top.iter().sum()
}

fn to_u64(source: &str) -> u64 {
    source.parse::<u64>().expect("Number was in an invalid format.")
}