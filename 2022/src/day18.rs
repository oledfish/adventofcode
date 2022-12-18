use std::{collections::{HashMap, VecDeque, HashSet}, hash::Hash};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64
}

impl Vector3 {
    fn from_slice(source: &str) -> Self {
        let mut parts = source.split(',');

        Self {
            x: parts.next().expect("No X coordinate.").parse::<i64>().expect("Invalid X coordinate."),
            y: parts.next().expect("No Y coordinate.").parse::<i64>().expect("Invalid Y coordinate."),
            z: parts.next().expect("No Z coordinate.").parse::<i64>().expect("Invalid Z coordinate."),
        }
    }

    fn adjacent(&self) -> [Self; 6] {
        let above  = Vector3 { x: self.x, y: self.y - 1, z: self.z };
        let below  = Vector3 { x: self.x, y: self.y + 1, z: self.z };
        let left   = Vector3 { x: self.x - 1, y: self.y, z: self.z };
        let right  = Vector3 { x: self.x + 1, y: self.y, z: self.z };
        let behind = Vector3 { x: self.x, y: self.y, z: self.z - 1 };
        let ahead  = Vector3 { x: self.x, y: self.y, z: self.z + 1 };

        [above, below, left, right, behind, ahead]
    }
}

fn main() {
    let input = include_str!("../input/day18.input");

    // Part one
    let surface_area = first_puzzle(input);
    println!("The surface area of the scanned lava droplet is {}.", surface_area);

    // Part two
    let surface_area_revised = second_puzzle(input);
    println!("The revised surface area of the scanned lava droplet is {}.", surface_area_revised);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day18.input");

    assert_eq!(first_puzzle(sample), 64);
    assert_eq!(second_puzzle(sample), 58);
}

fn first_puzzle(source: &str) -> u64 {
    total_visible_area(&parse_cubes(source))
}

fn second_puzzle(source: &str) -> u64 {
    let mut cubes = parse_cubes(source);

    fill_holes(&mut cubes);
    total_visible_area(&cubes)
}

fn fill_holes(cubes: &mut HashMap<Vector3, bool>) {
    let mut min = Vector3 { x: i64::MAX, y: i64::MAX, z: i64::MAX };
    let mut max = Vector3 { x: i64::MIN, y: i64::MIN, z: i64::MIN };

    for vector in cubes.keys() {
        min.x = min.x.min(vector.x);
        min.y = min.y.min(vector.y);
        min.z = min.z.min(vector.z);

        max.x = max.x.max(vector.x);
        max.y = max.y.max(vector.y);
        max.z = max.z.max(vector.z);
    }

    min.x -= 1;
    min.y -= 1;
    min.z -= 1;

    max.x += 1;
    max.y += 1;
    max.z += 1;

    for z in min.z..=max.z {
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let pos = Vector3 { x, y, z };

                // Three-dimensional flood fill
                if !cubes.contains_key(&pos) {
                    let mut visited: HashSet<Vector3> = HashSet::new();
                    let mut queue: VecDeque<Vector3> = VecDeque::new();
                    let mut hole = true;

                    queue.push_front(pos);
                    visited.insert(pos);

                    loop {
                        if queue.is_empty() || !hole {
                            break;
                        }

                        let current = queue.pop_back().unwrap();
                        for side in current.adjacent() {
                            if side.x == min.x || side.x == max.x || side.y == min.y || side.y == max.y || side.z == min.z || side.z == max.z {
                                hole = false;
                                break;
                            }

                            if !cubes.contains_key(&side) && !visited.contains(&side) {
                                queue.push_front(side);
                                visited.insert(side);
                            }
                        }
                    }

                    if hole {
                        for vector in visited {
                            cubes.insert(vector, true);
                        }
                    }
                }
            }
        }
    }
}

fn total_visible_area(cubes: &HashMap<Vector3, bool>) -> u64 {
    let mut total = 0;

    for pos in cubes.keys() {
        let mut count = 6;
        for vector in pos.adjacent() {
            if cubes.contains_key(&vector) {
                count -= 1
            }
        }

        total += count;
    }

    total
}

fn parse_cubes(source: &str) -> HashMap<Vector3, bool> {
    let mut cubes = HashMap::<Vector3, bool>::new();

    source
        .lines()
        .map(Vector3::from_slice)
        .for_each(|vector| {
            cubes.insert(vector, true);
        });

    cubes
}