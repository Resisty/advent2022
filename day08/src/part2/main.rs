fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_input_vec(opts.input_file.clone());
    let grid = lines.iter()
        .map(|l| l.chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect()
        ).collect::<Vec<Vec<i32>>>();
    let mut answer = grid.len() * 2 + (grid[0].len() - 2) * 2; // start with all trees on periphery;
    println!("Got grid:");
    println!("{grid:?}");
    println!("Visible trees before looking inward: {answer}");
    for i in 1..grid.len() - 1 {
        for j in  1..grid[i].len() - 1 {
            // start inside outermost box of trees
            let height = &grid[i][j];
            let row = &grid[i];
            let col = grid.iter().map(|l| l.iter().nth(j).unwrap()).collect::<Vec<_>>();
            let row_left_max = row[0..j].iter().max().unwrap();
            println!("Compare tree at ({i}, {j}) with height {height} to max of trees to LEFT: {row_left_max}");
            let row_right_max = row[j+1..row.len()].iter().max().unwrap();
            println!("Compare tree at ({i}, {j}) with height {height} to max of trees to RIGHT: {row_right_max}");
            let col_up_max = *col[0..i].iter().max().unwrap();
            println!("Compare tree at ({i}, {j}) with height {height} to max of trees to UP: {col_up_max}");
            let col_down_max = *col[i+1..col.len()].iter().max().unwrap();
            println!("Compare tree at ({i}, {j}) with height {height} to max of trees to DOWN: {col_down_max}");
            if height > row[0..j].iter().max().unwrap()
                || height > row[j+1..row.len()].iter().max().unwrap()
                || height > *col[0..i].iter().max().unwrap()
                || height > *col[i+1..col.len()].iter().max().unwrap() {
                    answer += 1;
                    println!("Tree at ({i}, {j}) with height {height} is VISIBLE");
            } else {
                println!("Tree at ({i}, {j}) with height {height} is NOT VISIBLE");
            }
        }
    }
    println!("Number of visible trees: {answer}");
}
