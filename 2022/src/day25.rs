trait Snafu {
    type T;
    const MINUS: char = '-';
    const MINUSMINUS: char = '=';

    fn from_snafu(input: &str) -> Self::T;
    fn to_snafu(input: Self::T) -> String;
    
    fn from_digit(input: char) -> Self::T;
    fn to_digit(input: Self::T) -> char;
}

impl Snafu for i64 {
    type T = i64;

    fn from_snafu(input: &str) -> i64 {
        input
            .chars()
            .rev()
            .enumerate()
            .fold(0, |result, (position, digit)| {
                result + Self::from_digit(digit) * 5_i64.pow(position as u32)
            })
    }

    fn to_snafu(input: Self::T) -> String {
        let mut num = input;
        
        let mut result = "".to_string();
        let mut leftover = 0;

        loop {
            if num == 0 && leftover == 0 {
                break;
            }

            let mut remainder = num % 5 + leftover;
            leftover = 0;

            if remainder == 3 || remainder == 4 {
                leftover += 1;
            } else if remainder >= 5 {
                leftover += remainder / 5;
                remainder %= 5;
            }

            result.insert(0, Self::to_digit(remainder));
            num /= 5;
        }

        result
    }

    fn from_digit(input: char) -> i64 {
        match input {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            Self::MINUS => -1,
            Self::MINUSMINUS => -2,
            _ => panic!("Invalid digit: '{}'", input)
        }
    }

    fn to_digit(input: i64) -> char {
        match input {
            0  => '0',
            1  => '1',
            2  => '2',
            3  => Self::MINUSMINUS,
            4  => Self::MINUS,
            -1 => Self::MINUS,
            -2 => Self::MINUSMINUS,
            _ => panic!("Invalid digit: '{}'", input)
        }
    }
}

fn main() {
    let input = include_str!("../input/day25.input");

    let snafu = solve(input);
    println!("The SNAFU number that must be supplied to Bob's console is '{}'.", snafu);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day25.input");

    assert_eq!(solve(sample), "2=-1=0");
}

fn solve(source: &str) -> String {
    i64::to_snafu(source.lines().map(i64::from_snafu).sum())
}

#[test]
fn conversion_test() {
    assert_eq!(i64::to_snafu(1), "1");
    assert_eq!(i64::to_snafu(2), "2");
    assert_eq!(i64::to_snafu(3), "1=");
    assert_eq!(i64::to_snafu(4), "1-");
    assert_eq!(i64::to_snafu(5), "10");
    assert_eq!(i64::to_snafu(6), "11");
    assert_eq!(i64::to_snafu(7), "12");
    assert_eq!(i64::to_snafu(8), "2=");
    assert_eq!(i64::to_snafu(9), "2-");
    assert_eq!(i64::to_snafu(10), "20");
    assert_eq!(i64::to_snafu(15), "1=0");
    assert_eq!(i64::to_snafu(20), "1-0");
    assert_eq!(i64::to_snafu(2022), "1=11-2");
    assert_eq!(i64::to_snafu(12345), "1-0---0");
    assert_eq!(i64::to_snafu(314159265), "1121-1110-1=0");

    assert_eq!(i64::from_snafu("1"), 1);
    assert_eq!(i64::from_snafu("2"), 2);
    assert_eq!(i64::from_snafu("1="), 3);
    assert_eq!(i64::from_snafu("1-"), 4);
    assert_eq!(i64::from_snafu("10"), 5);
    assert_eq!(i64::from_snafu("11"), 6);
    assert_eq!(i64::from_snafu("12"), 7);
    assert_eq!(i64::from_snafu("2="), 8);
    assert_eq!(i64::from_snafu("2-"), 9);
    assert_eq!(i64::from_snafu("20"), 10);
    assert_eq!(i64::from_snafu("1=0"), 15);
    assert_eq!(i64::from_snafu("1-0"), 20);
    assert_eq!(i64::from_snafu("1=11-2"), 2022);
    assert_eq!(i64::from_snafu("1-0---0"), 12345);
    assert_eq!(i64::from_snafu("1121-1110-1=0"), 314159265);
}