fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_input_vec(opts.input_file.clone());
    let mut x_vals: Vec<i32> = Vec::new();
    x_vals.push(1);
    let mut screen: Vec<char> = Vec::new();
    let mut cycle = 0;
    for line in lines {
        let prev = *x_vals.last().unwrap();
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["noop"] => {
                draw(prev, cycle, &mut screen);
                x_vals.push(prev);
                cycle += 1;
            }
            ["addx", n] => {
                draw(prev, cycle, &mut screen);
                x_vals.push(prev);
                cycle += 1;
                draw(prev, cycle, &mut screen);
                x_vals.push(prev + n.parse::<i32>().unwrap());
                cycle += 1;
            },
            _ => panic!("Unknown instruction: {line}")
        }
    }
    let answer = &screen[..].chunks(40).map(|l| l.iter().collect()).collect::<Vec<String>>();
    for line in answer {
        println!("{line}");
    }
}

fn draw(x: i32, cycle: i32, screen: &mut Vec<char>) {
    match x - (cycle % 40) {
        -1 | 0 | 1 => screen.push('#'),
        _ => screen.push('.')
    }
}
