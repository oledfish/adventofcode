fn main() {
    let input = include_str!("../input/day08.input");

    // Part one
    let visible_count = first_puzzle(input);
    println!("A total of {} trees are visible from outside the grid.", visible_count);

    // Part two
    let highest_score = second_puzzle(input);
    println!("The highest scenic score is {}.", highest_score);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day08.input");

    assert_eq!(first_puzzle(sample), 21);
    assert_eq!(second_puzzle(sample), 8);
}

fn first_puzzle(source: &str) -> usize{
    let grid = parse_grid(source);

    let width = grid[0].len();
    let height = grid.len();

    let mut count = 0;

    for i in 1..grid[0].len() - 1 {
        for j in 1..grid.len() - 1 {
            let tree = grid[j][i];

            let mut left_visible = true;
            let mut top_visible = true;
            let mut right_visible = true;
            let mut bot_visible = true;

            // Left
            for k in 0..i {
                let adjacent = grid[j][k];
                if adjacent >= tree {
                    left_visible = false;
                    break;
                }
            }

            // Top
            for k in 0..j {
                let adjacent = grid[k][i];
                if adjacent >= tree {
                    top_visible = false;
                    break;
                }
            }

            // Right
            for k in i+1..width {
                let adjacent = grid[j][k];
                if adjacent >= tree {
                    right_visible = false;
                    break;
                }
            }

            // Bottom
            for k in j+1..height {
                let adjacent = grid[k][i];
                if adjacent >= tree {
                    bot_visible = false;
                    break;
                }
            }

            if left_visible || top_visible || right_visible || bot_visible {
                count += 1;
            }
        }
    }
    
    count += width * 2; // Top and bottom edges are visible
    count += (height - 2) * 2; // Left and right edges are visible
    count
}

fn second_puzzle(source: &str) -> usize{
    let grid = parse_grid(source);

    let width = grid[0].len();
    let height = grid.len();

    let mut max_score = 0;

    for i in 1..grid[0].len() - 1 {
        for j in 1..grid.len() - 1 {
            let tree = grid[j][i];

            let mut left_visible = 0;
            let mut top_visible = 0;
            let mut right_visible = 0;
            let mut bot_visible = 0;

            // Left
            for k in (0..i).rev() {
                let adjacent = grid[j][k];

                left_visible += 1;
                if adjacent >= tree {
                    break;
                }
            }

            // Top
            for k in (0..j).rev() {
                let adjacent = grid[k][i];
                top_visible += 1;

                if adjacent >= tree {
                    break;
                }
            }

            // Right
            for k in i+1..width {
                let adjacent = grid[j][k];

                right_visible += 1;
                if adjacent >= tree {
                    break;
                }
            }

            // Bottom
            for k in j+1..height {
                let adjacent = grid[k][i];

                bot_visible += 1;
                if adjacent >= tree {
                    break;
                }
            }

            let scenic_score = left_visible * top_visible * right_visible * bot_visible;

            if scenic_score > max_score {
                max_score = scenic_score
            }
        }
    }
    
    max_score
}

fn parse_grid(source: &str) -> Vec<Vec<u32>> {
    let mut grid = vec![vec![]; 0];

    source
        .lines()
        .enumerate()
        .for_each(|(index, line)| {
            grid.push(vec![]);

            for c in line.chars() {
                grid[index].push(c.to_digit(10).unwrap())
            }
        });

    grid
}