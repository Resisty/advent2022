fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let vec_lines = adventils::get_input_vec(opts.input_file.clone());
    let elf_sacks = vec_lines.split(|l| *l == "");
    let elf_calories = elf_sacks.map(calories_per_elf).collect::<Vec<_>>();
    println!("Most elf calories: {}", elf_calories.iter().max().unwrap());
}

fn calories_per_elf(sack: &[String]) -> i32 {
    sack.iter().map(|s| s.parse::<i32>().unwrap()).sum()
}
