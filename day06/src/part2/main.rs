use std::collections::HashSet;

const MESSAGE: usize = 14;

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_raw_input(opts.input_file.clone());
    let chars = lines.lines().next().unwrap().chars().collect::<Vec<char>>();
    let mut index: i32 = -1;
    for i in 0usize..chars.len() - MESSAGE {
        let substring = &chars[i..i+MESSAGE];
        let sub: HashSet<&char> = HashSet::from_iter(substring.clone());
        match sub {
            _ if sub.len() == MESSAGE => {
                index = i as i32 + MESSAGE as i32;
                break
            }
            _ => {
                continue
            }
        };
    }
    println!("Start of packet: {index}");
}
