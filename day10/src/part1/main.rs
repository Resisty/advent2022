fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_input_vec(opts.input_file.clone());
    let mut x_vals: Vec<i32> = Vec::new();
    x_vals.push(1);
    for line in lines {
        let prev = *x_vals.last().unwrap();
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["noop"] => {
                x_vals.push(prev);
            }
            ["addx", n] => {
                x_vals.push(prev);
                x_vals.push(prev + n.parse::<i32>().unwrap());
            },
            _ => panic!("Unknown instruction: {line}")
        }
    }
    let answer: i32 = x_vals.iter().enumerate().filter(|(i, _)| *i + 1 == 20 || (*i + 21) % 40 == 0).map(|(i, x)| (i as i32 + 1) * x).collect::<Vec<i32>>().iter().sum();
    println!("Signal strength sum: {answer}");
}
