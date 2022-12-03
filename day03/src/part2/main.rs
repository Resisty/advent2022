fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let vec_lines = adventils::get_input_vec(opts.input_file.clone());
    let three_elf_groups = vec_lines.chunks(3);
    let total: i32 = three_elf_groups.map(|l| find_common(l)).map(|c| get_priority(c)).sum();
    println!("Total: {total}");
}

fn find_common(lines: &[String]) -> char {
    let (first, second, third) = (lines[0].clone(), lines[1].clone(), lines[2].clone());
    let filtered: Vec<char> = first.chars().filter(|c| second.contains(&c.to_string()) && third.contains(&c.to_string())).collect();
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
