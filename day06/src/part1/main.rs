use std::collections::HashSet;

const PACKET: usize = 4;

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_raw_input(opts.input_file.clone());
    let chars = lines.lines().next().unwrap().chars().collect::<Vec<char>>();
    let mut index: i32 = -1;
    for i in 0usize..chars.len() - PACKET {
        let sub = &chars[i..i+PACKET];
        let sub: HashSet<&char> = HashSet::from_iter(sub.clone());
        match sub {
            _ if sub.len() == PACKET => {
                index = i as i32 + PACKET as i32;
                break
            }
            _ => {
                continue
            }
        };
    }
    println!("Start of packet: {index}");
}
