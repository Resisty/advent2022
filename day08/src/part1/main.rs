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
    for i in 1..grid.len() - 1 {
        for j in  1..grid[i].len() - 1 {
            // start inside outermost box of trees
            let height = &grid[i][j];
            let row = &grid[i];
            let col = grid.iter().map(|l| l.iter().nth(j).unwrap()).collect::<Vec<_>>();
            if height > row[0..j].iter().max().unwrap()
                || height > row[j+1..row.len()].iter().max().unwrap()
                || height > *col[0..i].iter().max().unwrap()
                || height > *col[i+1..col.len()].iter().max().unwrap() {
                    answer += 1;
            }
        }
    }
    println!("Number of visible trees: {answer}");
}
