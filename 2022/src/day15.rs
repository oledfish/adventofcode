type Position = (i64, i64);

fn main() {
    let input = include_str!("../input/day15.input");

    let beacon_not_here = first_puzzle(input, 2000000);
    println!("In the row at y=2000000, the beacon can't be in {} positions.", beacon_not_here);

    let tuning_frequency = second_puzzle(input, 4000000);
    println!("The tuning frequency of the distress signal is {}.", tuning_frequency);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day15.input");

    assert_eq!(first_puzzle(sample, 10), 26);
    assert_eq!(second_puzzle(sample, 20), 56000011);
}

fn first_puzzle(source: &str, y: i64) -> usize {
    let associations = parse_map_and_associations(source);

    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for (sensor, beacon) in &associations {
        let distance = manhattan(sensor, beacon) as i64;

        if sensor.1 - distance < min {
            min = sensor.1 - distance;
        }

        if sensor.1 + distance > max {
            max = sensor.1 + distance;
        }
    }

    let mut positions = 0;

    for x in min..max {
        let try_point = (x, y);

        for (sensor, beacon) in &associations {
             let beacon_distance = manhattan(sensor, beacon);
             let point_distance = manhattan(sensor, &try_point);

             if point_distance <= beacon_distance {
                positions += 1;
                break;
             }
        }
    }

    positions - 1
}

fn second_puzzle(source: &str, max: i64) -> u64 {
    let associations = parse_map_and_associations(source);
    
    let mut position = None;

    for (sensor, beacon) in &associations {
        let distance = manhattan(sensor, beacon) as i64;

        let sensor_x = sensor.0;
        let sensor_y = sensor.1;

        // To find the signal, we must iterate over the positions right outside
        // the diamond formed by a sensor and a beacon.
        // Unfortunately this looks fairly ugly.

        let left = sensor_x - distance - 1..=sensor_x;
        let up = sensor_y - distance - 1..=sensor_y;
        let right = sensor_x..=sensor_x + distance + 1;
        let down = sensor_y..=sensor_y + distance + 1;

        let sides = std::iter::zip(left.clone(), up.clone().rev())
            .chain(std::iter::zip(right.clone(), up.clone()))
            .chain(std::iter::zip(left.clone(), down.clone()))
            .chain(std::iter::zip(right.clone(), down.clone().rev()));

        for outside in sides {
            if outside.0 <= 0 || outside.0 >= max || outside.1 <= 0 || outside.1 >= max {
                continue;
            }

            if position.is_some() {
                break;
            }

            let signal = check_boundary_point(&outside, sensor, &associations);
            if signal {
                position = Some(outside);
                break;
            }
        }
    }

    if let Some(p) = position {
        return p.0 as u64 * 4000000 + p.1 as u64;
    }

    panic!("Couldn't find distress signal.");
}

fn manhattan(a: &Position, b: &Position) -> u64 {
    (a.0 - b.0).unsigned_abs() + (a.1 - b.1).unsigned_abs()
}

fn check_boundary_point(outside: &Position, origin_sensor: &Position, associations: &[(Position, Position)]) -> bool {
    associations
        .iter()
        .filter(|(sensor, _)| sensor != origin_sensor)
        .all(|(sensor, beacon)| {
            if sensor == origin_sensor {
                return true;
            }

            let beacon_distance = manhattan(sensor, beacon);
            let point_distance = manhattan(sensor, outside);

            point_distance > beacon_distance
        })
}

fn parse_map_and_associations(source: &str) -> Vec<(Position, Position)> {
    let mut pairs = vec![];

    source
        .lines()
        .map(parse_line)
        .for_each(|(sensor, beacon)| {
            pairs.push((sensor, beacon));
        });

    pairs
}

fn parse_line(source: &str) -> (Position, Position) {
    let mut parts = source.split(": ");

    let sensor_str = parts
        .next()
        .expect("Found end of string.")
        .strip_prefix("Sensor at ")
        .expect("Invalid format for sensor information.");

    let sensor = parse_position(sensor_str);

    let beacon_str = parts
        .next()
        .expect("Found end of string.")
        .strip_prefix("closest beacon is at ")
        .expect("Invalid format for beacon information.");

    let beacon =  parse_position(beacon_str);

    (sensor, beacon)
}

fn parse_position(source: &str) -> Position {
    let mut parts = source.split(", ");
    
    let x = parts.next().expect("Found end of string.")
        .strip_prefix("x=")
        .expect("Invalid format for X coordinate.")
        .parse::<i64>()
        .expect("Invalid number.");

    let y = parts.next().expect("Found end of string.")
        .strip_prefix("y=")
        .expect("Invalid format for Y coordinate.")
        .parse::<i64>()
        .expect("Invalid number.");

    (x, y)
}