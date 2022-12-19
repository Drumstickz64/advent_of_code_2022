use itertools::Itertools;

pub fn solve_part_one(input: String) -> String {
    let tree_grid: Vec<Vec<TreeHeight>> = parse_input(&input);
    let grid_height = tree_grid.len();
    let grid_width = tree_grid[0].len();
    tree_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let tree_grid = &tree_grid;
            row.iter()
                .copied()
                .enumerate()
                .map(move |(x, tree_height)| {
                    let left_hidden = (0..x).any(|left_x| {
                        let left_tree_height = tree_grid[y][left_x];
                        left_tree_height >= tree_height
                    });

                    if !left_hidden {
                        return true;
                    }

                    let top_hidden = (0..y).any(|top_y| {
                        let top_tree_height = tree_grid[top_y][x];
                        top_tree_height >= tree_height
                    });

                    if !top_hidden {
                        return true;
                    }

                    let right_hidden = ((x + 1)..grid_width).any(|right_x| {
                        let right_tree_height = tree_grid[y][right_x];
                        right_tree_height >= tree_height
                    });

                    if !right_hidden {
                        return true;
                    }

                    let bottom_hidden = ((y + 1)..grid_height).any(|bottom_y| {
                        let bottom_tree_height = tree_grid[bottom_y][x];
                        bottom_tree_height >= tree_height
                    });

                    if !bottom_hidden {
                        return true;
                    }

                    false
                })
        })
        .filter(|&b| b)
        .count()
        .to_string()
}

pub fn solve_part_two(input: String) -> String {
    let tree_grid: Vec<Vec<TreeHeight>> = parse_input(&input);
    let grid_height = tree_grid.len();
    let grid_width = tree_grid[0].len();
    tree_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let tree_grid = &tree_grid;
            row.iter()
                .copied()
                .enumerate()
                .map(move |(x, tree_height)| {
                    let mut left_score = 0;
                    for left_x in (0..x).rev() {
                        left_score += 1;
                        let left_tree_height = tree_grid[y][left_x];
                        if tree_height <= left_tree_height {
                            break;
                        }
                    }

                    let mut top_score = 0;
                    for top_y in (0..y).rev() {
                        top_score += 1;
                        let top_tree_height = tree_grid[top_y][x];
                        if tree_height <= top_tree_height {
                            break;
                        }
                    }
                    let mut right_score = 0;
                    for right_x in (x + 1)..grid_width {
                        right_score += 1;
                        let right_tree_height = tree_grid[y][right_x];
                        if tree_height <= right_tree_height {
                            break;
                        }
                    }
                    let mut bottom_score = 0;
                    #[allow(clippy::needless_range_loop)]
                    for bottom_y in (y + 1)..grid_height {
                        bottom_score += 1;
                        let bottom_tree_height = tree_grid[bottom_y][x];
                        if tree_height <= bottom_tree_height {
                            break;
                        }
                    }

                    left_score * top_score * right_score * bottom_score
                })
        })
        .max()
        .unwrap()
        .to_string()
}

type TreeHeight = u8;

fn parse_input(input: &str) -> Vec<Vec<TreeHeight>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec()
}
