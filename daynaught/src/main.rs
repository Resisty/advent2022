fn main() {
    let opts = adventils::Options::args();
    println!("Used adventils::Options::args to get program options and arguments...");
    println!("Verbosity: {}", opts.verbose);
    println!("Input file: {}", opts.input_file);
    let contents = adventils::get_input(opts.input_file.clone());
    println!("Contents of file {}:\n{}", opts.input_file, contents);
    println!("Contents of file {} as lines:", opts.input_file);
    for line in contents.lines() {
        println!("\t{line}");
    }
    println!("Contents of file {} as lines, tokenized on ' ':", opts.input_file);
    for line in contents.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        println!("\ttokens:");
        for token in tokens {
            println!("\t\t{token}");
        }
    }
}
