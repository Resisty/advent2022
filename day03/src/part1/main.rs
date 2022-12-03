fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let vec_lines = adventils::get_input_vec(opts.input_file.clone());
    let total: i32 = vec_lines.iter().map(|l| find_common(&l)).map(get_priority).sum();
    println!("Total: {total}");
}

fn find_common(line: &String) -> char {
    let (left, right) = line.split_at(line.len() / 2);
    let filtered: Vec<char> = left.chars().filter(|c| right.contains(&c.to_string())).collect();
    return filtered[0]
}

fn get_priority(c: char) -> i32 {
    let a = c as i32;
    let result = match a {
        _ if a >= 97 => a - 96,
        _ if a >= 65 => a - 38,
        _ => {
            println!("You fucked up");
            0
        }
    };
    return result
}
