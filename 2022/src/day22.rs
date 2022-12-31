use std::{collections::HashMap, hash::Hash};

#[derive(Default)]
struct Cube {
    faces: HashMap<usize, Face>,
    size: i64
}

struct Face {
    row: i64,
    col: i64,
    rotation: i64,
    map: HashMap<Vector2, Tile>,
    mappings: HashMap<Direction, (Direction, usize)>,
    inner_mappings: HashMap<Direction, Direction>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    East,
    North,
    West,
    South
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }
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
            (-1,  0) => Direction::West,
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
    let password_cube = second_puzzle(input);
    println!("The password derived from the cube map is {}.", password_cube);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day22.input");

    assert_eq!(first_puzzle(sample), 6032);
    assert_eq!(second_puzzle(sample), 5031);
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

fn second_puzzle(source: &str) -> i64 {
    let (cube, path, start) = parse_cube(source);

    let mut current = start;

    let mut delta = Vector2 { x: 1, y: 0 };
    let mut face = cube.faces.get(&1).unwrap();

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

                        let (new, next_index) = *face.mappings.get(&dir).unwrap();
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
                                face = next_face;
                                delta = next_delta;
                                current = target;
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

fn parse_cube(source: &str) -> (Cube, Vec<Command>, Vector2) {
    /*
        General process:

        1.  Parse the map as in the first puzzle, as a flat map.

        2.  Build the local positions of each tile in each face, using the flat map

        3.  Build a net with the paths from any face to any other face.

        4.  Start with a base Rotation3 for each face, and from face number "1", perform
            rotations depending on which directions we are moving and store for each face.

        5.  Face 1 is an arbitrary start and has a normal of x:0, y:0, z: 1. Applying
            the corresponding rotation for each face will result in the corresponding normals
            for all 6 faces. There are no duplicates.

        6.  We must assume a cube with unrotated faces. By rotating the vertices of an unrotated face 1,
            we would get the positions of the vertices for the other faces as if the face itself wasn't
            rotated. We assume the cube is size 1. Corresponding this way, with 1 as the "front" face:

            Back face: Rotating 180 degrees by the Y axis.
            Top face: Rotating -90 degrees by the X axis.
            Bottom face: Rotating 90 degrees by the X axis.
            Left face: Rotating -90 degrees by the Y axis.
            Right face: Rotating 90 degrees by the Y axis.

        7.  The faces of the cube we're parsing are probably rotated in comparison to the cube we're assuming,
            so with the information we have, we take again the vertices for an unrotated face 1, apply the
            rotations described above, then for each face repeat the process with the roations we stored in
            step 5. The vertices will not necessarily match. We perform an array shift as many times as needed
            until they match, which will correspond to the rotation (in degrees) along the normal axis. 

        8.  Each face has "inner mappings" which map the expected unrotated cardinal point to the actual cardinal
            point in comparison. For unrotated faces, this is east => east, west => west, etc. With the local
            rotation found in step 7, we modify the mappings to match.

        9.  The outer mappings ("moving from a cardinal point in this face, in which point of which face we will arrive")
            are built by querying faces by their normal, taking into account the inner mappings of both the origin
            face and the adjacent face.

        10. The cube is folded
    
    */

    // Face width/height
    let size = ((source.chars().filter(|c| *c == '.' || *c == '#').count() as f64) / 6.0).sqrt() as i64;
    let mut cube = Cube { size, ..Default::default() };

    for index in 1..=6 {
        let face = Face {
            row: 0,
            col: 0,
            rotation: 0,
            map: HashMap::<Vector2, Tile>::new(),
            mappings: HashMap::<Direction, (Direction, usize)>::new(),
            inner_mappings: HashMap::from([
                (Direction::East, Direction::East),
                (Direction::North, Direction::North),
                (Direction::West, Direction::West),
                (Direction::South, Direction::South),
            ])
        };

        cube.faces.insert(index, face);
    }

    let (full_map, path, mut start) = parse_input(source);

    let mut max = Vector2::default();
    for pos in full_map.keys() {
        max.x = max.x.max(pos.x + 1);
        max.y = max.y.max(pos.y + 1);
    }

    let rows = max.y / size;

    let mut face = 1;
    let mut layout = [[0; 4]; 4];

    for row in 0..rows {
        let start_y = row * size;
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

                    layout[(chunk_y / size) as usize][(chunk_x / size) as usize] = face;

                    if global_pos == start {
                        start = local_pos;
                    }
                }
            }

            start_x += size;
            face += 1;
        }
    }

    #[derive(Copy, Clone, Default)]
    struct Adjacenct {
        left: Option<usize>,
        right: Option<usize>,
        up: Option<usize>,
        down: Option<usize>
    }

    let mut unresolved = HashMap::<usize, Adjacenct>::new();
    for i in 1..=6 {
        let face = cube.faces.get(&i).unwrap();
        let row = face.row;
        let col = face.col;

        unresolved.insert(i, Adjacenct::default());

        let adj = unresolved.get_mut(&i).unwrap();
        if let Some((adj_left, _)) = cube.faces.iter().find(|(_, face)| face.row == row && face.col == col - 1) {
            adj.left = Some(*adj_left);
        }

        if let Some((adj_right, _)) = cube.faces.iter().find(|(_, face)| face.row == row && face.col == col + 1) {
            adj.right = Some(*adj_right);
        }

        if let Some((adj_up, _)) = cube.faces.iter().find(|(_, face)| face.row == row - 1 && face.col == col) {
            adj.up = Some(*adj_up);
        }

        if let Some((adj_down, _)) = cube.faces.iter().find(|(_, face)| face.row == row + 1 && face.col == col) {
            adj.down = Some(*adj_down);
        }
    }

    let x_axis = nalgebra::Vector3::<f64>::x_axis();
    let y_axis = nalgebra::Vector3::<f64>::y_axis();
    let z_axis = nalgebra::Vector3::<f64>::z_axis();

    let left_rot = nalgebra::Rotation3::<f64>::from_axis_angle(&y_axis, f64::to_radians(-90.0));
    let right_rot = nalgebra::Rotation3::<f64>::from_axis_angle(&y_axis, f64::to_radians(90.0));
    let up_rot = nalgebra::Rotation3::<f64>::from_axis_angle(&x_axis, f64::to_radians(-90.0));
    let down_rot = nalgebra::Rotation3::<f64>::from_axis_angle(&x_axis, f64::to_radians(90.0));

    let vert_ne = nalgebra::Vector3::new( 1.0,  1.0, 0.0);
    let vert_se = nalgebra::Vector3::new( 1.0, -1.0, 0.0);
    let vert_sw = nalgebra::Vector3::new(-1.0, -1.0, 0.0);
    let vert_nw = nalgebra::Vector3::new(-1.0,  1.0, 0.0);

    // Front side of cube
    let mut verts_front = [
        vert_ne,
        vert_se,
        vert_sw,
        vert_nw
    ];

    // Back side of cube
    let rotation = right_rot * right_rot;
    let mut verts_back = [
        rotation.transform_vector(&vert_ne),
        rotation.transform_vector(&vert_se),
        rotation.transform_vector(&vert_sw),
        rotation.transform_vector(&vert_nw),
    ];

    // Left side of cube
    let rotation = left_rot;
    let mut verts_left = [
        rotation.transform_vector(&vert_ne),
        rotation.transform_vector(&vert_se),
        rotation.transform_vector(&vert_sw),
        rotation.transform_vector(&vert_nw),
    ];

    // Right side of cube
    let rotation = right_rot;
    let mut verts_right = [
        rotation.transform_vector(&vert_ne),
        rotation.transform_vector(&vert_se),
        rotation.transform_vector(&vert_sw),
        rotation.transform_vector(&vert_nw),
    ];

    // Up side of cube
    let rotation = up_rot;
    let mut verts_up = [
        rotation.transform_vector(&vert_ne),
        rotation.transform_vector(&vert_se),
        rotation.transform_vector(&vert_sw),
        rotation.transform_vector(&vert_nw),
    ];

    // Down side of cube
    let rotation = down_rot;
    let mut verts_down = [
        rotation.transform_vector(&vert_ne),
        rotation.transform_vector(&vert_se),
        rotation.transform_vector(&vert_sw),
        rotation.transform_vector(&vert_nw),
    ];

    for side in [&mut verts_front, &mut verts_back, &mut verts_left, &mut verts_right, &mut verts_up, &mut verts_down].iter_mut() {
        for vert in side.iter_mut() {
            vert.x = vert.x.round();
            vert.y = vert.y.round();
            vert.z = vert.z.round();
        }
    }

    // Vector3<f64> must be turned into (i64, i64, i64) because f64 doesn't implement Eq
    let mut faces = HashMap::<(i64, i64, i64), (usize, nalgebra::Rotation3<f64>)>::new();

    let mut visited = vec![];
    let mut stack = vec![];
    visited.push(1);
    stack.push((1, nalgebra::Rotation3::identity()));

    loop {
        if stack.is_empty() {
            break;
        }

        let (current_face, matrix) = stack.pop().unwrap();
        let normal = nalgebra::Vector3::<f64>::z();
        let rotated = matrix.transform_vector(&normal);

        faces.insert((rotated.x as i64, rotated.y as i64, rotated.z as i64), (current_face, matrix));

        let current_adj = &unresolved.get(&current_face).unwrap();

        if let Some(left) = current_adj.left {
            if !visited.contains(&left) {
                visited.push(left);
                stack.push((left, matrix * left_rot));
            }
        }

        if let Some(right) = current_adj.right {
            if !visited.contains(&right) {
                visited.push(right);
                stack.push((right, matrix * right_rot));
            }
        }

        if let Some(up) = current_adj.up {
            if !visited.contains(&up) {
                visited.push(up);
                stack.push((up, matrix * up_rot));
            }
        }

        if let Some(down) = current_adj.down {
            if !visited.contains(&down) {
                visited.push(down);
                stack.push((down, matrix * down_rot));
            }
        }
    }

    for (normal, (face_num, matrix)) in &faces {
        let (normalized_verts, _) = match normal {
            ( 0,  0,  1) => (&verts_front, &z_axis),
            ( 0,  0, -1) => (&verts_back, &z_axis),
            ( 0,  1,  0) => (&verts_up, &y_axis),
            ( 0, -1,  0) => (&verts_down, &y_axis),
            ( 1,  0,  0) => (&verts_right, &x_axis),
            (-1,  0,  0) => (&verts_left, &x_axis),
            _ => unreachable!()
        };

        let mut local_verts = [
            matrix.transform_vector(&vert_ne),
            matrix.transform_vector(&vert_se),
            matrix.transform_vector(&vert_sw),
            matrix.transform_vector(&vert_nw),
        ];

        for vert in local_verts.iter_mut() {
            vert.x = vert.x.round();
            vert.y = vert.y.round();
            vert.z = vert.z.round();
        }

        let mut grid_rotation = 0.0;
        if local_verts != *normalized_verts {
            loop {
                if local_verts == *normalized_verts {
                    break;
                }

                grid_rotation += 90.0;
                local_verts.rotate_left(1);
            }
        }

        let face = cube.faces.get_mut(face_num).unwrap();
        face.rotation = grid_rotation as i64;
    }

    for (_, (face_num, _)) in &faces {
        if cube.faces.get(face_num).unwrap().rotation != 0 {
            let mut rotation = cube.faces.get(face_num).unwrap().rotation;

            loop {
                if rotation == 0 {
                    break;
                }

                let face = cube.faces.get_mut(face_num).unwrap();
                for (_, dir) in face.inner_mappings.iter_mut() {
                    *dir = Direction::rotate(&dir);
                };

                rotation -= 90;
            }
        }
    }

    for (normal, (this_face_num, _)) in &faces {
        let associations = match normal {
            ( 0,  0,  1) => {[
                (Direction::East, Direction::West, faces.get(&( 1,  0,  0)).unwrap().0),
                (Direction::North, Direction::South, faces.get(&( 0,  1,  0)).unwrap().0),
                (Direction::West, Direction::East, faces.get(&(-1,  0,  0)).unwrap().0),
                (Direction::South, Direction::North, faces.get(&( 0, -1,  0)).unwrap().0)
            ]},

            ( 0,  0, -1) => {[
                (Direction::East, Direction::West, faces.get(&(-1,  0,  0)).unwrap().0),
                (Direction::North, Direction::North, faces.get(&( 0,  1,  0)).unwrap().0), 
                (Direction::West, Direction::East, faces.get(&( 1,  0,  0)).unwrap().0), 
                (Direction::South, Direction::South, faces.get(&( 0, -1,  0)).unwrap().0)  
            ]},

            ( 0,  1,  0) => {[
                (Direction::East, Direction::North, faces.get(&( 1,  0,  0)).unwrap().0),
                (Direction::North, Direction::North, faces.get(&( 0,  0, -1)).unwrap().0), 
                (Direction::West, Direction::North, faces.get(&(-1,  0,  0)).unwrap().0), 
                (Direction::South, Direction::North, faces.get(&( 0,  0,  1)).unwrap().0)
            ]},

            ( 0, -1,  0) => {[
                (Direction::East, Direction::South, faces.get(&( 1,  0,  0)).unwrap().0), 
                (Direction::North, Direction::South, faces.get(&( 0,  0,  1)).unwrap().0),  
                (Direction::West, Direction::South, faces.get(&(-1,  0,  0)).unwrap().0),
                (Direction::South, Direction::South, faces.get(&( 0,  0, -1)).unwrap().0)
            ]},

            ( 1,  0,  0) => {[
                (Direction::East, Direction::West, faces.get(&( 0,  0, -1)).unwrap().0),
                (Direction::North, Direction::East, faces.get(&( 0,  1,  0)).unwrap().0),  
                (Direction::West, Direction::East, faces.get(&( 0,  0,  1)).unwrap().0),  
                (Direction::South, Direction::East, faces.get(&( 0, -1,  0)).unwrap().0)
            ]},

            (-1,  0,  0) => {[
                (Direction::East, Direction::West, faces.get(&( 0,  0,  1)).unwrap().0),
                (Direction::North, Direction::West, faces.get(&( 0,  1,  0)).unwrap().0), 
                (Direction::West, Direction::East, faces.get(&( 0,  0, -1)).unwrap().0),
                (Direction::South, Direction::West, faces.get(&( 0, -1,  0)).unwrap().0)
            ]},

            _ => unreachable!()
        };

        for (this_dir, other_dir, other_face_num) in associations {
            let other_face = cube.faces.get(&other_face_num).unwrap();
            let other_true_dir = *other_face.inner_mappings.get(&other_dir).unwrap();

            let this_face = cube.faces.get_mut(this_face_num).unwrap();
            let this_true_dir = *this_face.inner_mappings.get(&this_dir).unwrap();

            this_face.mappings.insert(this_true_dir, (other_true_dir, other_face_num));
        }
    }

    (cube, path, start)
}