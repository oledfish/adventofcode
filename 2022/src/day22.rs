use std::{collections::HashMap, hash::Hash};

enum CubeShape {
    Example,
    Input
}

impl CubeShape {
    fn mappings(&self, index: usize) -> Mappings {
        match (self, index) {
            // Real input shape
            (Self::Input, 1) => Mappings {
                east: (Direction::West, 2),
                north: (Direction::West, 6),
                west: (Direction::West, 4),
                south: (Direction::North, 3),
            },

            (Self::Input, 2) => Mappings {
                east: (Direction::East, 5),
                north: (Direction::South, 6),
                west: (Direction::East, 1),
                south: (Direction::East, 3),
            },

            (Self::Input, 3) => Mappings {
                east: (Direction::South, 2),
                north: (Direction::South, 1),
                west: (Direction::North, 4),
                south: (Direction::North, 5),
            },

            (Self::Input, 4) => Mappings {
                east: (Direction::West, 5),
                north: (Direction::West, 3),
                west: (Direction::West, 1),
                south: (Direction::North, 6),
            },

            (Self::Input, 5) => Mappings {
                east: (Direction::East, 2),
                north: (Direction::South, 3),
                west: (Direction::East, 4),
                south: (Direction::East, 6),
            },

            (Self::Input, 6) => Mappings {
                east: (Direction::South, 5),
                north: (Direction::South, 4),
                west: (Direction::North, 1),
                south: (Direction::North, 2),
            },

            // Example shape
            (Self::Example, 1) => Mappings {
                east: (Direction::East, 6),
                north: (Direction::North, 2),
                west: (Direction::North, 3),
                south: (Direction::North, 4),
            },

            (Self::Example, 2) => Mappings {
                east: (Direction::West, 3),
                north: (Direction::North, 1),
                west: (Direction::South, 6),
                south: (Direction::South, 5),
            },

            (Self::Example, 3) => Mappings {
                east: (Direction::West, 4),
                north: (Direction::West, 1),
                west: (Direction::East, 2),
                south: (Direction::West, 5),
            },

            (Self::Example, 4) => Mappings {
                east: (Direction::North, 6),
                north: (Direction::South, 1),
                west: (Direction::East, 3),
                south: (Direction::North, 5),
            },

            (Self::Example, 5) => Mappings {
                east: (Direction::West, 6),
                north: (Direction::South, 4),
                west: (Direction::South, 3),
                south: (Direction::South, 2),
            },

            (Self::Example, 6) => Mappings {
                east: (Direction::East, 1),
                north: (Direction::East, 4),
                west: (Direction::East, 5),
                south: (Direction::West, 2),
            },

            _ => panic!("Invalid mapping")
        }
    }
}

#[derive(Default)]
struct Cube {
    faces: HashMap<usize, Face>,
    size: i64
}

struct Face {
    row: i64,
    col: i64,
    map: HashMap<Vector2, Tile>,
    mappings: Mappings,
}

struct Mappings {
    east: (Direction, usize),
    north: (Direction, usize),
    west: (Direction, usize),
    south: (Direction, usize)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    North,
    West,
    South
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
struct Vector2 {
    x: i64,
    y: i64
}

impl Vector2 {
    fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    fn rotate(&mut self, command: &Command) {
        let (sin, cos) = match command {
            Command::RotateLeft => (-1, 0),
            Command::RotateRight => (1, 0),
            _ => panic!("Invalid operation")
        };

        let x = self.x;
        let y = self.y;
        
        self.x = cos * x - sin * y;
        self.y = sin * x + cos * y;
    }

    fn facing(&self) -> i64 {
        match (self.x, self.y) {
            ( 1,  0) => 0,
            ( 0,  1) => 1,
            (-1,  0) => 2,
            ( 0, -1) => 3,
            _ => panic!("Invalid direction")
        }
    }

    fn dir(&self) -> Direction {
        match (self.x, self.y) {
            ( 1,  0) => Direction::East,
            ( 0,  1) => Direction::South,
            (-1,  0) => Direction::East,
            ( 0, -1) => Direction::North,
            _ => panic!("Invalid direction")
        }
    }
}

// A lack of a tile is a wall and thus a wrap around
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Floor,
    Wall,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Command {
    RotateRight,
    RotateLeft,
    Move(i64)
}

fn main() {
    let input = include_str!("../input/day22.input");

    // Part one
    let password_flat = first_puzzle(input);
    println!("The password derived from the flat map is {}.", password_flat);

    // Part two
    let password_cube = second_puzzle(input, CubeShape::Input);
    println!("The password derived from the cube map is {}.", password_cube);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day22.input");

    assert_eq!(first_puzzle(sample), 6032);
    assert_eq!(second_puzzle(sample, CubeShape::Example), 5031);
}

fn first_puzzle(source: &str) -> i64 {
    let (map, path, start) = parse_input(source);

    let mut current = start;
    let mut delta = Vector2 { x: 1, y: 0 };

    for command in path {
        match command {
            Command::Move(num) => {
                for _ in 1..=num {
                    let mut target = current;
                    target.x += delta.x;
                    target.y += delta.y;

                    // Wrap-around
                    if !map.contains_key(&target) {
                        let mut search = delta;
                        search.invert();

                        target.x += search.x;
                        target.y += search.y;

                        loop {
                            if !map.contains_key(&target) {
                                target.x -= search.x;
                                target.y -= search.y;
                                break;
                            }

                            target.x += search.x;
                            target.y += search.y;
                        }
                    }

                    let tile = map.get(&target).unwrap();

                    match tile {
                        Tile::Floor => { current = target },
                        Tile::Wall => break
                    }
                }
            },

            Command::RotateRight => { 
                delta.rotate(&Command::RotateRight) 
            }

            Command::RotateLeft => {
                delta.rotate(&Command::RotateLeft) 
            }
        }
    }

    let row = current.y + 1;
    let col = current.x + 1;
    let facing = delta.facing();

    1000 * row + 4 * col + facing
}

fn second_puzzle(source: &str, shape: CubeShape) -> i64 {
    let (cube, path, start) = parse_cube(source, shape);

    let mut current = Vector2::default();
    let mut delta = Vector2 { x: 1, y: 0 };
    let mut index = 1;

    let mut face = cube.faces.get(&index).unwrap();

    for command in path {
        match command {
            Command::Move(num) => {
                for _ in 1..=num {
                    let mut target = current;
                    target.x += delta.x;
                    target.y += delta.y;

                    // Wrap-around
                    if !face.map.contains_key(&target) {
                        let dir = delta.dir();

                        let (new, next_index) = match dir {
                            Direction::East => face.mappings.east,
                            Direction::North => face.mappings.north,
                            Direction::West => face.mappings.west,
                            Direction::South => face.mappings.south,
                        };

                        let edge = cube.size - 1;
                        
                        target = match (dir, new) {
                            (Direction::East, Direction::East)  => Vector2 { x: edge, y: edge - target.y },
                            (Direction::East, Direction::West)  => Vector2 { x: 0, y: target.y },
                            (Direction::East, Direction::North) => Vector2 { x: edge - target.y, y: 0 },
                            (Direction::East, Direction::South) => Vector2 { x: target.y, y: edge },

                            (Direction::West, Direction::East)  => Vector2 { x: edge, y: target.y },
                            (Direction::West, Direction::West)  => Vector2 { x: 0, y: edge - target.y },
                            (Direction::West, Direction::North) => Vector2 { x: target.y, y: 0 },
                            (Direction::West, Direction::South) => Vector2 { x: edge - target.y, y: edge },

                            (Direction::North, Direction::East)  => Vector2 { x: edge, y: target.x },
                            (Direction::North, Direction::West)  => Vector2 { x: 0, y: target.x },
                            (Direction::North, Direction::North) => Vector2 { x: edge - target.x, y: 0 },
                            (Direction::North, Direction::South) => Vector2 { x: target.x, y: edge },

                            (Direction::South, Direction::East)  => Vector2 { x: edge, y: target.x },
                            (Direction::South, Direction::West)  => Vector2 { x: 0, y: target.x },
                            (Direction::South, Direction::North) => Vector2 { x: target.x, y: 0 },
                            (Direction::South, Direction::South) => Vector2 { x: edge - target.x, y: edge },
                        };

                        let next_delta = match new {
                            Direction::East  => Vector2 { x: -1, y: 0 },
                            Direction::West  => Vector2 { x: 1, y: 0},
                            Direction::North => Vector2 { x: 0, y: 1 },
                            Direction::South => Vector2 { x: 0, y: -1 },
                        };

                        let next_face = cube.faces.get(&next_index).unwrap();
                        let tile = next_face.map.get(&target).unwrap();

                        match tile {
                            Tile::Floor => {
                                index = next_index;
                                face = next_face;
                                delta = next_delta;
                                current = target 
                            },

                            Tile::Wall => break
                        }
                    } else {
                        let tile = face.map.get(&target).unwrap();

                        match tile {
                            Tile::Floor => { current = target },
                            Tile::Wall => break
                        }
                    }
                }
            },

            Command::RotateRight => { 
                delta.rotate(&Command::RotateRight) 
            }

            Command::RotateLeft => {
                delta.rotate(&Command::RotateLeft) 
            }
        }
    }

    let row = face.row * cube.size + current.y + 1;
    let col = face.col * cube.size + current.x + 1;
    let facing = delta.facing();

    1000 * row + 4 * col + facing
}

fn parse_input(source: &str) -> (HashMap<Vector2, Tile>, Vec<Command>, Vector2) {
    let mut map = HashMap::<Vector2, Tile>::new();
    let mut path = vec![];
    let mut start = None;
    
    let (map_slice, path_slice) = source
        .split_once("\n\n")
        .expect("Invalid input.");

    map_slice
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, tile)| {
                    let pos = Vector2 { x: x as i64, y: y as i64};

                    if !tile.is_ascii_whitespace() {
                        match tile {
                            '.' => map.insert(pos, Tile::Floor),
                            '#' => map.insert(pos, Tile::Wall),
                            _ => panic!("Invalid tile")
                        };
                    }

                    if start.is_none() && tile == '.' {
                        start = Some(pos);
                    }
                })
        });

    path_slice
        .replace('R', " R ")
        .replace('L', " L ")
        .split_ascii_whitespace()
        .for_each(|command| {
            match command {
                "R" => path.push(Command::RotateRight),
                "L" => path.push(Command::RotateLeft),
                _ => {
                    if let Ok(number) = command.parse::<i64>() {
                        path.push(Command::Move(number));
                    } else {
                        panic!("Invalid path.")
                    }
                }
            };
        });

    (map, path, start.unwrap())
}

fn parse_cube(source: &str, shape: CubeShape) -> (Cube, Vec<Command>, Vector2) {
    // Face width/height
    let size = ((source.chars().filter(|c| *c == '.' || *c == '#').count() as f64) / 6.0).sqrt() as i64;
    let mut cube = Cube { size, ..Default::default() };

    for index in 1..=6 {
        let face = Face {
            row: 0,
            col: 0,
            map: HashMap::<Vector2, Tile>::new(),
            mappings: shape.mappings(index)
        };

        cube.faces.insert(index, face);
    }

    let (full_map, path, start) = parse_input(source);

    let mut max = Vector2::default();
    for pos in full_map.keys() {
        max.x = max.x.max(pos.x + 1);
        max.y = max.y.max(pos.y + 1);
    }

    let rows = max.y / size;

    let mut face = 1;

    for row in 0..rows {
        let mut start_y = row * size;
        let mut start_x = 0;

        for x in 0..=max.x {
            let pos = Vector2 { x, y: start_y };
            if full_map.contains_key(&pos) {
                start_x = x;
                break;
            }
        }

        loop {
            let pos = Vector2 { x: start_x, y: start_y };

            if !full_map.contains_key(&pos) || start_x > max.x {
                break;
            }

            for chunk_x in start_x..start_x + size {
                for chunk_y in start_y..start_y + size {
                    let global_pos = Vector2 { x: chunk_x, y: chunk_y };
                    let local_pos = Vector2 { x: chunk_x - start_x, y: chunk_y - start_y};

                    let tile = full_map.get(&global_pos).unwrap();
                    let target = cube.faces.get_mut(&face).unwrap();

                    target.map.insert(local_pos, *tile);
                    target.row = row;
                    target.col = chunk_x / size;
                }
            }

            start_x += size;
            face += 1;
        }
    }

    (cube, path, start)
}