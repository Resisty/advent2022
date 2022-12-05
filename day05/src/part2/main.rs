use std::fmt;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CrateBox {
    column: usize,
    name: char
}

impl fmt::Display for CrateBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{column: {}, name: {}}}", self.column, self.name)
    }
}

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_raw_input(opts.input_file.clone());
    let parts: Vec<&str> = lines.split("\n\n").collect();
    let (crate_lines, instruction_lines) = (parts[0], parts[1]);
    let mut crates: Vec<Vec<CrateBox>> = Vec::new();
    let parsed_crates: Vec<Vec<CrateBox>> = crate_lines.lines()
        .map(|line| line.chars().enumerate()
            .filter(|(_, cha)| cha.is_alphabetic())
            .map(|(ind, cha)| CrateBox{column: ((ind)-1)/4, name: cha})
        .collect::<Vec<CrateBox>>())
        .filter(|vec| !vec.is_empty())
        .rev()
        .collect();
    for crate_row in parsed_crates {
        for cratebox in crate_row {
            while crates.len() <= cratebox.column {
                crates.push(Vec::new());
            }
            crates[cratebox.column].push(cratebox);
        }
    }
    let instructions: Vec<Vec<i32>> = instruction_lines.lines()
        .map(|line| line.split(' ').enumerate()
            .filter(|(ind, _)| ind % 2 == 1)
            .map(|(_, word)| word.parse().unwrap())
            .collect::<Vec<i32>>()
        ).collect();
    println!("{:?}", crates);
    println!("---");
    println!("{:?}", instructions);
    println!("---");
    for instruct in instructions {
        let (num, src, dest) = (instruct[0], instruct[1], instruct[2]);
        // I feel like this should be a lot easier with vec::drain()
        // but it just errors really unhelpfully:
        // "value of type `&mut Vec<CrateBox>` cannot be built from `std::iter::Iterator<Item=CrateBox>`"
        let mut tmpstack = Vec::new();
        for _ in 0..num {
            let the_crate = crates[src as usize - 1].pop().unwrap();
            tmpstack.push(the_crate);
        }
        tmpstack.reverse();
        crates[dest as usize - 1].append(&mut tmpstack);
    }
    println!("{:?}", crates);
    let msg = crates.iter().map(|cratebox| cratebox.last().unwrap().name.to_string()).collect::<Vec<String>>().join("");
    println!("Message: {msg}");
}
