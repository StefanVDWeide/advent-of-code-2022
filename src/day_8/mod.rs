use std::fs;

// TODO: Clean up code - split in functions
pub fn solution() {
    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string("src/day_8/input.txt").expect("Error during reading the input");

    // Trim the end of the file to ensure no \n will remain to screw you over later
    let trimmed_input = input.trim_end();

    let mut tree_grid: Vec<Vec<i32>> = trimmed_input
        .lines()
        .map(|line| {
            line.split_terminator("")
                .skip(1)
                .map(|value| value.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut visible_count = 0;
    let mut scenic_score_list: Vec<i32> = vec![];

    for (row_index, row) in tree_grid.iter().enumerate() {
        for (column_index, tree) in row.iter().enumerate() {
            // The very first tree and the very last tree on the edges are always visisble

            if row_index == 0
                || row_index == tree_grid.len() - 1
                || column_index == 0
                || column_index == row.len() - 1
            {
                visible_count += 1;
                continue;
            }

            let mut largerst_left_of_index = true;
            let mut largerst_right_of_index = true;
            let mut largerst_up_of_index = true;
            let mut largerst_down_of_index = true;

            let mut scenic_score_left = 0;
            let mut scenic_score_right = 0;
            let mut scenic_score_up = 0;
            let mut scenic_score_down = 0;

            // Check left of curent index
            for item in &row[0..column_index] {
                if item >= tree {
                    largerst_left_of_index = false;
                    break;
                }
            }
            // Check right of the index
            for item in &row[column_index + 1..] {
                if item >= tree {
                    largerst_right_of_index = false;
                    break;
                }
            }

            // Check up of index
            for item in &tree_grid[0..row_index] {
                if item[column_index] >= *tree {
                    largerst_up_of_index = false;
                    break;
                }
            }
            // Check down of index
            for item in &tree_grid[row_index + 1..] {
                if item[column_index] >= *tree {
                    largerst_down_of_index = false;
                    break;
                }
            }

            // #########################################################
            // Part 2

            let reverse_left_values = &row[0..column_index].to_vec();
            // Check left of curent index
            for item in reverse_left_values.iter().rev() {
                scenic_score_left += 1;
                if item >= tree {
                    break;
                }
            }
            // Check right of the index
            for item in &row[column_index + 1..] {
                scenic_score_right += 1;
                if item >= tree {
                    break;
                }
            }

            let reverse_up_values = &tree_grid[0..row_index].to_vec();
            // Check up of index
            for item in reverse_up_values.iter().rev() {
                scenic_score_up += 1;
                if item[column_index] >= *tree {
                    break;
                }
            }
            // Check down of index
            for item in &tree_grid[row_index + 1..] {
                scenic_score_down += 1;
                if item[column_index] >= *tree {
                    break;
                }
            }

            if largerst_left_of_index
                || largerst_right_of_index
                || largerst_up_of_index
                || largerst_down_of_index
            {
                visible_count += 1;
            }

            let total_scenic_score =
                scenic_score_left * scenic_score_right * scenic_score_up * scenic_score_down;

            scenic_score_list.push(total_scenic_score);
        }
    }

    println!("The visible count for the trees is: {visible_count}");
    println!(
        "The highest scenic score is: {:?}",
        scenic_score_list.iter().max()
    )
}
