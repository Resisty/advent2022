use std::cmp::Ordering;

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum PacketKind {
    List(Vec<PacketKind>),
    Int(i32)
}

impl PartialOrd for PacketKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketKind {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketKind::Int(left), PacketKind::Int(right)) => {
                left.cmp(right)
            },
            (PacketKind::List(left), PacketKind::List(right)) => {
                for (leftitem, rightitem) in left.iter().zip(right) {
                    let comparison = leftitem.cmp(rightitem);
                    if comparison != Ordering::Equal {
                        return comparison
                    }
                }
                left.len().cmp(&right.len())
            },
            (PacketKind::List(left), PacketKind::Int(right)) => {
                let mut container: Vec<PacketKind> = Vec::new();
                container.push(PacketKind::Int(right.clone()));
                left.cmp(&container)
            },
            (PacketKind::Int(left), PacketKind::List(right)) => {
                let mut container: Vec<PacketKind> = Vec::new();
                container.push(PacketKind::Int(left.clone()));
                container.cmp(right)
            }
        }
    }
}

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_raw_input(opts.input_file.clone());
    let pairs = lines.split("\n\n").map(|l| l.trim().split("\n").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let mut packets: Vec<PacketKind> = Vec::new();
    let div1 = String::from("[[2]]");
    let (div1parse, _) = s_to_packets(div1.as_str());
    packets.push(div1parse);
    let div2 = String::from("[[6]]");
    let (div2parse, _) = s_to_packets(div2.as_str());
    packets.push(div2parse);
    for pair in pairs {
        for packet in pair {
            let (parsed, _) = s_to_packets(packet);
            packets.push(parsed);
        }
    }
    packets.sort();
    let (div1parse, _) = s_to_packets(div1.as_str());
    let (div2parse, _) = s_to_packets(div2.as_str());
    let ind1 = packets.iter().position(|i| *i == div1parse).unwrap();
    let ind2 = packets.iter().position(|i| *i == div2parse).unwrap();
    println!("Decoder key: {} * {} = {}", ind1 + 1, ind2 + 1, (ind1 + 1) * (ind2 + 1) );
}

fn s_to_packets(s: &str) -> (PacketKind, &str) {
    let cdr = s.strip_prefix('[');
    match cdr {
        Some(mut right) => {
            // If we stripped off a '[', we're at the start of a list.
            // Create a new vector to store the items in the list
            let mut packet = Vec::new();
            loop {
                let cddr = right.strip_prefix(']');
                match cddr {
                    Some(done) => {
                        // If we stripped off a ']', we've finished adding items to the current
                        // packet. Return up the stack.
                        break (PacketKind::List(packet), done)
                    },
                    None => () // keep going
                }
                // We have items to add, so recurse (see this match-level's None, line #)
                let (car, cdddr) = s_to_packets(right);
                // car must be a PacketKind, so push it onto the current vector
                packet.push(car);
                // The item may have been a list of a single numberical item, ending in ']'
                // check it
                let done = cdddr.strip_prefix(']');
                match done {
                    Some(ok) => {
                        // Same as above, if we stripped off a ']', we've finished adding items.
                        // Return.
                        break (PacketKind::List(packet), ok)
                    },
                    None => () // keep going, same as above
                }
                // If we're not done with the list, we must have remaining numerical items to parse
                let cddddr = cdddr.strip_prefix(',');
                match cddddr {
                    Some(items) => {
                        right = items;
                        continue;
                    },
                    None => {
                        panic!("This shouldn't be possible, cddddr didn't start with ',': {s}")
                    }
                }
            }
        }
        None => {
            // We're somewhere at the start of a numerical item having already started a vector
            // Find the first index of the first of any of ',', '[', ']', defaulting to the end of
            // the string, marking the end of a numerical item
            let stop = s.find(|c| matches!(c, ',' | '[' | ']')).unwrap_or(s.len());
            let (car, cdr) = s.split_at(stop);
            // Parse the numerical item into an actual number and send the cdr back up the stack
            (PacketKind::Int(car.parse().unwrap()), cdr)
        }
    }
}
