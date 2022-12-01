fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    let contents = adventils::get_input(opts.input_file.clone());
    // implement solutions in here somewhere
    let mut vec = Vec::new();
    let mut sum: i32 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            vec.push(sum);
            sum = 0;
            continue;
        }
        let line_num = line.parse::<i32>().unwrap();
        sum += line_num;
    }
    vec.push(sum); // Push the last sum
    let max_val = vec.iter().max();
    println!("Max value: {}", max_val.unwrap().to_string());
}
