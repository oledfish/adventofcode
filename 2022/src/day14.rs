const AIR: u8 = 0;
const ROCK: u8 = 1;
const SAND: u8 = 2;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn from_slice(source: &str) -> Point {
        let mut nums = source.split(',');

        let x = nums
            .next()
            .expect("Needed an X coordinate.")
            .parse::<usize>()
            .expect("Invalid number format.");

        let y = nums
            .next()
            .expect("Needed a Y coordinate.")
            .parse::<usize>()
            .expect("Invalid number format.");

        Point { x, y }
    }
}

fn main() {
    let input = include_str!("../input/day14.input");

    // Part one
    let sand_void = first_puzzle(input);
    println!("There are {} units of sand accumulated before it starts falling into the abyss.", sand_void);

    // Part two
    let sand_floor = second_puzzle(input);
    println!("There are {} units of sand accumulated when there is a floor.", sand_floor);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day14.input");

    assert_eq!(first_puzzle(sample), 24);
    assert_eq!(second_puzzle(sample), 93);
}

fn first_puzzle(source: &str) -> usize {
    let (mut waterfall, left_offset) = build_waterfall(source);

    let mut sand_point = Point { x: 500 - left_offset, y: 0 };
    let mut sand = 0;

    loop {
        let landed = drop_sand(&mut waterfall, &mut sand_point, false);

        if landed.is_some() {
            sand += 1;
            continue;
        }

        break;
    }

    sand
}

fn second_puzzle(source: &str) -> usize {
    let (mut waterfall, left_offset) = build_waterfall(source);
    waterfall.push(vec![AIR; waterfall.first().unwrap().len()]);
    waterfall.push(vec![ROCK; waterfall.first().unwrap().len()]);

    let mut sand_point = Point { x: 500 - left_offset, y: 0 };
    let mut sand = 0;

    loop {
        let landed = drop_sand(&mut waterfall, &mut sand_point, true);

        if let Some(landed_point) = landed {
            sand += 1;

            if landed_point.x == sand_point.x && landed_point.y == sand_point.y {
                break;
            }
        }
    }

    sand
}

fn build_waterfall(source: &str) -> (Vec<Vec<u8>>, usize) {
    let paths: Vec<Vec<Point>> = source
        .lines()
        .map(parse_path)
        .collect();
        
    let mut max_width = 0;
    let mut min_width = usize::MAX;
    let mut max_height = 0;

    for path in &paths {
        for point in path {
            if point.x > max_width {
                max_width = point.x;
            }

            if point.x < min_width {
                min_width = point.x;
            }

            if point.y > max_height {
                max_height = point.y;
            }
        }
    }

    let mut waterfall = vec![vec![AIR; (max_width - min_width) + 1]; max_height + 1];

    for path in &paths {
        let len = path.len();

        for i in 1..len {
            let start = Point { x: path[i-1].x - min_width, y: path[i-1].y };
            let end = Point { x: path[i].x - min_width, y: path[i].y };

            if start.x != end.x && start.y != end.y {
                panic!("Cannot create paths that aren't horizontal or diagonal.");
            }

            if start.x != end.x {
                for j in if start.x < end.x { start.x..=end.x } else { end.x..=start.x } {
                    waterfall[start.y][j] = ROCK;
                }
            }

            if start.y != end.y {
                for j in if start.y < end.y { start.y..=end.y } else { end.y..=start.y } {
                    waterfall[j][start.x] = ROCK;
                }
            }
        }
    }

    (waterfall, min_width)
}

fn drop_sand(waterfall: &mut Vec<Vec<u8>>, point: &mut Point, extend: bool) -> Option<Point> {
    let mut current = *point;
    let mut target = *point;
    let min_width = 0;
    let max_width = waterfall[target.y + 1].len();
    let max_height = waterfall.len();

    loop {
        if target.y > max_height {
            return None;
        }

        if waterfall[target.y + 1][target.x] >= ROCK {
            if target.x == min_width {
                if extend {
                    extend_waterfall(waterfall, -1);
                    target.x += 1;
                    point.x += 1;
                } else {
                    return None
                };
            }

            if waterfall[target.y + 1][target.x - 1] >= ROCK {
                if target.x + 1 >= max_width {
                    if extend {
                        extend_waterfall(waterfall, 1);
                    } else {
                        return None
                    };
                }

                if waterfall[target.y + 1][target.x + 1] >= ROCK {
                    waterfall[target.y][target.x] = SAND;
                    return Some(target);
                } else {
                    target.y += 1;
                    target.x += 1;
                }
            } else {
                target.y += 1;
                target.x -= 1;
            }
        } else {
            target.y += 1;
        }
    }
}

fn extend_waterfall(waterfall: &mut Vec<Vec<u8>>, side: i8) {
    let width = waterfall.len() - 1;
    for (index, row) in waterfall.iter_mut().enumerate() {
        let element = if index != width { AIR } else { ROCK };

        if side < 0 {
            row.insert(0, element);
        } else {
            row.push(element);
        }
    }
}
fn parse_path(source: &str) -> Vec<Point> {
    source
        .split(" -> ")
        .map(Point::from_slice)
        .collect()
}

/*fn print_waterfall(waterfall: &Vec<Vec<u8>>) {
    for row in waterfall {
        for point in row {
            match *point {
                AIR => print!("."),
                ROCK => print!("#"),
                SAND => print!("o"),
                _ => unreachable!()
            }
        }

        println!();
    }
}*/
