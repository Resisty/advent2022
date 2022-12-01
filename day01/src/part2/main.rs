fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let vec_lines = adventils::get_input_vec(opts.input_file.clone());
    let elf_sacks = vec_lines.split(|l| *l == "");
    let mut elf_calories = elf_sacks.map(calories_per_elf).collect::<Vec<_>>();
    elf_calories.sort();
    let top_three: i32 = elf_calories.iter().rev().take(3).sum();
    println!("Top three snack holders' calories summed together: {}", top_three.to_string());
}

fn calories_per_elf(sack: &[String]) -> i32 {
    sack.iter().map(|s| s.parse::<i32>().unwrap()).sum()
}
