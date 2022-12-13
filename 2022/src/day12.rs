use std::collections::VecDeque;

const LEFT: usize = 0;
const UP: usize = 1;
const RIGHT: usize = 2;
const DOWN: usize = 3;

#[derive(Debug, Default, Copy, Clone)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Debug, Default, Copy, Clone)]
struct Terrain {
    height: u64,
    adjacent: [Option<Point>; 4]
}

#[derive(Debug, Default, Copy, Clone)]
struct TerrainAux {
    visited: bool,
    distance: u64,
    preceding: Option<Point>
}


struct Map {
    data: Vec<Vec<Terrain>>,
    width: usize,
    height: usize
}

trait PointAddressable {
    type Value;

    fn at(&self, point: &Point) -> &Self::Value;
    fn at_mut(&mut self, point: &Point) -> &mut Self::Value;
}

impl<T> PointAddressable for Vec<Vec<T>> {
    type Value = T;

    fn at(&self, point: &Point) -> &T {
        &self[point.y as usize][point.x as usize]
    }

    fn at_mut(&mut self, point: &Point) -> &mut T {
        &mut self[point.y as usize][point.x as usize]
    }
}

fn main() {
    let input = include_str!("../input/day12.input");

    let steps_start = first_puzzle(input);
    println!("The goal can be reached from the start in as little as {} steps.", steps_start);

    let steps_a = second_puzzle(input);
    println!("The goal can be reached from any square with 'a' elevation in as little as {} steps.", steps_a);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day12.input");
    
    assert_eq!(first_puzzle(sample), 31);
    assert_eq!(second_puzzle(sample), 29);
}

fn first_puzzle(source: &str) -> usize {
    let (map, start, end) = parse_map(source);
    shortest_path(&map, &start, &end).unwrap()
}

fn second_puzzle(source: &str) -> usize {
    let (map, _, end) = parse_map(source);
    let from_elevation = elevation('a');

    let mut paths = Vec::new();

    for y in 0..map.height {
        for x in 0..map.width {
            if map.data[y][x].height != from_elevation {
                continue;
            }

            let start = Point { x: x as i64, y: y as i64 };
            paths.push(shortest_path(&map, &start, &end).unwrap_or(usize::MAX));
        }
    }

    paths.sort();
    *paths.first().unwrap()
}

// Breadth-first search
fn shortest_path(map: &Map, start: &Point, end: &Point) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut found = false;

    let mut info = vec![vec![TerrainAux { visited: false, distance: u64::MAX, preceding: None }; map.width]; map.height];

    queue.push_back(*start);
    let mut start_info = info.at_mut(start);
    start_info.distance = 0;
    start_info.visited = true;

    loop {
        if queue.is_empty() {
            break;
        }

        let from_point = queue.pop_front().unwrap();
        let from_terrain = map.data.at(&from_point);
        let from_info = *info.at(&from_point);

        let adjacent = from_terrain.adjacent;

        for adj in adjacent.into_iter().flatten() {
            let to_info = info.at_mut(&adj);

            if !to_info.visited {
                to_info.visited = true;
                to_info.distance = from_info.distance + 1;
                to_info.preceding = Some(from_point);

                queue.push_back(adj);

                if adj.x == end.x && adj.y == end.y {
                    found = true;
                    break;
                }
            }
        }
    }

    if !found {
        return None;
    }

    let mut path = Vec::new();
    let mut current = *end;
    path.push(current);

    loop {
        let terrain = info.at(&current);
        if terrain.preceding.is_none() {
            break;
        }

        let preceding = terrain.preceding.unwrap();
        path.push(preceding);
        current = preceding;
    }

    Some(path.len() - 1)
}


fn parse_map(source: &str) -> (Map, Point, Point) {
    let mut peeking = source.lines().peekable();
    let map_width = peeking.peek().unwrap().len();
    let map_height = peeking.count();

    let mut map = Map {
        data: vec![vec![Terrain::default(); map_width]; map_height],
        width: map_width,
        height: map_height
    };

    let mut start = Point::default();
    let mut end = Point::default();

    source
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, height)| {
                    map.data[y][x] = Terrain {
                        height: elevation(height),
                        adjacent: [None; 4],
                    };

                    if height == 'S' {
                        start.x = x as i64;
                        start.y = y as i64;
                    }

                    if height == 'E' {
                        end.x = x as i64;
                        end.y = y as i64;
                    }
                });
        });


    for y in 0..map.height {
        for x in 0..map.width {
            let from = Point { x: x as i64, y: y as i64 };

            let left = Point { x: from.x - 1, y: from.y };
            if can_travel(&map, &from, &left) {
                map.data[y][x].adjacent[LEFT] = Some(left);
            }

            let up = Point { x: from.x, y: from.y - 1 };
            if can_travel(&map, &from, &up) {
                map.data[y][x].adjacent[UP] = Some(up);
            }

            let right = Point { x: from.x + 1, y: from.y };
            if can_travel(&map,&from, &right) {
                map.data[y][x].adjacent[RIGHT] = Some(right);
            }

            let down = Point { x: from.x, y: from.y + 1 };
            if can_travel(&map, &from, &down) {
                map.data[y][x].adjacent[DOWN] = Some(down);
            }
        }
    }

    (map, start, end)
}

fn can_travel(map: &Map, current: &Point, destination: &Point) -> bool {
    if destination.x < 0 || destination.y < 0 || destination.x > (map.width as i64 - 1) || destination.y > (map.height as i64 - 1) {
        return false;
    }

    let current_height = map.data.at(current).height;
    let destination_height = map.data.at(destination).height;

    if current_height >= destination_height {
        true
    } else {
        destination_height - current_height == 1
    }
}

fn elevation(character: char) -> u64 {
    match character {
        character @ 'a'..='z' => character as u64 - 96,
        'E' => 27,
        'S' => 1,
        _ => panic!("Invalid character.")
    }
}