use std::cmp::min;

fn part_two(input: &str) {}
fn part_one(input: &str) {
    let mut grand_sum = 0;
    let lines: Vec<_> = input.lines().collect();
    let length = lines.first().unwrap().len();
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    for line in lines {
        let grid_row: Vec<char> = line.chars().collect();
        grid.push(grid_row);
    }
    let mut numbers = vec![];
    for (i, row) in grid.iter().enumerate() {
        let mut current_number: Option<Vec<(usize, usize)>> = None;
        for (j, character) in row.iter().enumerate() {
            if character.is_digit(10) {
                if let Some(ref mut current) = current_number {
                    current.push((i, j))
                } else {
                    current_number = Some(vec![(i, j)])
                }
            } else if let Some(current) = current_number {
                numbers.push(current);
                current_number = None;
            }
        }

        if let Some(current) = current_number {
            numbers.push(current);
        }
    }
    for number_coords in numbers {
        let (start_x, start_y) = number_coords.first().unwrap().clone();
        let y_search_start = match start_y {
            0 => 0,
            x => x - 1
        };
        let y_search_end = min(start_y + number_coords.len() + 1, length);

        let x_search_start = match start_x {
            0 => 0,
            x => x - 1
        };
        let x_search_end = if start_x == grid.len() - 1 { start_x + 1 } else { start_x + 2 };

        let mut adjacent_to_symbol = false;
        for i in (x_search_start..x_search_end) {
            for j in (y_search_start..y_search_end) {
                let val = grid[i][j];

                if !val.is_digit(10) && val != '.' {
                    adjacent_to_symbol = true;
                    break;
                }
            }
            if adjacent_to_symbol { break; }
        }
        if adjacent_to_symbol {
            let mut value = 0;
            for (x, y) in number_coords {
                let place_value = grid[x][y].to_digit(10).unwrap();
                value *= 10;
                value += place_value;

            }
            grand_sum += value;
        }
    }
    println!("{grand_sum}")
}

pub fn run() {
    let input = include_str!("../data/day3input.txt");

    part_one(input);
    part_two(input);
}