fn main() {
    let input = include_str!("../input/day04.input");

    // Part one
    let full_overlaps = first_puzzle(input);
    println!("There are {} assignment pairs where one range fully contains the other.", full_overlaps);

    // Part two
    let partial_overlaps = second_puzzle(input);
    println!("There are {} assignment pairs where the ranges overlap.", partial_overlaps);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day04.input");

    assert_eq!(first_puzzle(sample), 2);
    assert_eq!(second_puzzle(sample), 4);
}

fn first_puzzle(source: &str) -> usize {
    source
        .lines()
        .map(get_ranges)
        .filter(|(range1, range2)| full_overlap(range1, range2))
        .count()
}

fn second_puzzle(source: &str) -> usize {
    source
        .lines()
        .map(get_ranges)
        .filter(|(range1, range2)| partial_overlap(range1, range2))
        .count()
}

fn get_ranges(source: &str) -> ([usize; 2], [usize; 2]) {
    let mut ranges_str = source.split(',');
    
    let mut range1 = ranges_str.next().unwrap().split('-');
    let mut range2 = ranges_str.next().unwrap().split('-');

    let start1 = range1.next().unwrap().parse::<usize>().unwrap();
    let end1 = range1.next().unwrap().parse::<usize>().unwrap();

    let start2 = range2.next().unwrap().parse::<usize>().unwrap();
    let end2 = range2.next().unwrap().parse::<usize>().unwrap();

    ([start1, end1], [start2, end2])
}

fn full_overlap(range1: &[usize; 2], range2: &[usize; 2]) -> bool {
    (range1[0] <= range2[0] && range1[1] >= range2[1]) ||
    (range2[0] <= range1[0] && range2[1] >= range1[1])
}

fn partial_overlap(range1: &[usize; 2], range2: &[usize; 2]) -> bool {
    range1[1] >= range2[0] && range2[1] >= range1[0]
}