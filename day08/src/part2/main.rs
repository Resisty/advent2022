fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_input_vec(opts.input_file.clone());
    let grid = lines.iter()
        .map(|l| l.chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect()
        ).collect::<Vec<Vec<i32>>>();
    let mut visibilities: Vec<usize> = Vec::new();
    for i in 0..grid.len() {
        for j in  0..grid[i].len() {
            // start inside outermost box of trees
            let mut vis: usize = 1;
            let height = grid[i][j];
            let mut row = grid[i].clone();
            let row_len = row.len().clone();
            let mut col = grid.iter().map(|l| l.iter().nth(j).unwrap().clone()).collect::<Vec<_>>();
            let col_len = col.len().clone();
            let row_left_max = row[0..j].iter().max().unwrap_or(&-1).clone();
            let row_right_max = row[j+1..row.len()].iter().max().unwrap_or(&-1).clone();
            let col_up_max = col[0..i].iter().max().unwrap_or(&&-1).clone();
            let col_down_max = col[i+1..col.len()].iter().max().unwrap_or(&&-1).clone();
            match row_left_max {
                -1 => vis *= 0,
                _ => {
                    let split_to_max = row[0..j].rsplitn_mut(2, |h| *h >= height).collect::<Vec<_>>();
                    vis *= match split_to_max {
                        _ if split_to_max.len() > 1 => split_to_max[0].len() + 1,
                        _  => split_to_max[0].len()
                    }
                }
            }
            match row_right_max {
                -1 => vis *= 0,
                _ => {
                    let split_to_max = row[j+1..row_len].splitn_mut(2, |h| *h >= height).collect::<Vec<_>>();
                    vis *= match split_to_max {
                        _ if split_to_max.len() > 1 => split_to_max[0].len() + 1,
                        _  => split_to_max[0].len()
                    }
                }
            }
            match col_up_max {
                -1 => vis *= 0,
                _ => {
                    let split_to_max = col[0..i].rsplitn_mut(2, |h| *h >= height).collect::<Vec<_>>();
                    vis *= match split_to_max {
                        _ if split_to_max.len() > 1 => split_to_max[0].len() + 1,
                        _  => split_to_max[0].len()
                    }
                }
            }
            match col_down_max {
                -1 => vis *= 0,
                _ => {
                    let split_to_max = col[i+1..col_len].splitn_mut(2, |h| *h >= height).collect::<Vec<_>>();
                    vis *= match split_to_max {
                        _ if split_to_max.len() > 1 => split_to_max[0].len() + 1,
                        _  => split_to_max[0].len()
                    }
                }
            }
            visibilities.push(vis);
        }
    }
    let answer = visibilities.iter().max().unwrap();
    println!("Number of visible trees: {answer}");
}
