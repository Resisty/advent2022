use std::fmt;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Range {
    min: i32,
    max: i32
}

#[allow(dead_code)]
impl Range {
    fn new(min: i32, max: i32) -> Range {
        Range {min, max}
    }
    fn from_string(s: &str) -> Range {
        let ranges: Vec<&str> = s.split('-').collect();
        let (min, max) = (ranges[0], ranges[1]);
        Range {min: min.parse().unwrap(), max: max.parse().unwrap()}
    }
    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{min: {}, max: {}}}", self.min, self.max)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pair {
    first: Range,
    second: Range
}

impl Pair {
    fn from_string(s: String) -> Pair {
        let ranges: Vec<&str> = s.split(',').collect();
        let (left, right) = (ranges[0], ranges[1]);
        Pair {first: Range::from_string(left), second: Range::from_string(right)}
    }
    fn contains(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }
}

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let vec_lines = adventils::get_input_vec(opts.input_file.clone());
    let pairs: Vec<Pair> = vec_lines.iter().map(|l| Pair::from_string(l.to_string())).collect();
    let overlaps = pairs.iter().filter(|p| p.contains()).count();
    println!("Total number of contained/containing pairs: {overlaps}");
}
