use std::fs;

fn main() {
    let list = fs::read_to_string("input/day01.input").expect("Error when reading file.");
    let mut calories = vec![];

    let mut count = 0;
    let mut max = 0;

    // Part one
    for line in list.lines() {
        if line.len() == 0 {
            calories.push(count);

            if count > max {
                max = count;
            }

            count = 0;
            continue;
        }

        let value = u64::from_str_radix(line, 10).expect("Number was in an invalid format.");
        count += value;
    };

    println!("The elf carrying the most calories has {} calories.", max);

    // Part two, reusing data from the previous loop
    calories.sort_by(|a, b| b.cmp(a));

    let sum: u64 = calories[0..3].iter().sum();
    println!("The top three elves are carrying {} calories in total.", sum);
}
