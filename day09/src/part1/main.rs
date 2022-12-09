use std::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash)]
pub struct Knot {
    x: i32,
    y: i32
}

impl Knot {
    pub fn move_pt(&mut self, other: &Knot) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn tail_catch_up(&mut self, head: &Knot) {
        // the tail plays catchup to head
        let x_dist = head.x - self.x;
        let x_sign = if x_dist < 0 { -1 } else { 1 };
        let y_dist = head.y - self.y;
        let y_sign = if y_dist < 0 { -1 } else { 1 };
        match (x_dist * x_sign, y_dist * y_sign) {
            // head is on tail or laterally/diagonally adjacent to tail. No move.
            (0, 0) | (0, 1) | (1, 0) | (1, 1) => (),
            // head has moved vertically twice; tail moves vertically in same direction
            (0, 2) => self.y += y_sign,
            // head has moved horizontally twice; tail moves horizontally
            (2, 0) => self.x += x_sign,
            // head is either one off on x-axis but two off on y-axis OR
            // head is one off on y-axis but two off on x-axis OR
            // head and tail are two off on both axes (previous tail moved diagonally)
            // tail moves diagonally
            (1, 2) | (2, 1) | (2, 2) => {
                self.x += x_sign;
                self.y += y_sign;
            },
            _ => {
                panic!("Unaccounted-for manhattan distance between head ({head}) and tail({self}): {x_dist}, {y_dist}");
            }
        }
    }
}

impl PartialEq for Knot {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y 
    }
}
impl Eq for Knot {}

impl fmt::Display for Knot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

pub struct Rope {
    knots: Vec<Knot>
}

impl Rope {
    pub fn update(&mut self, start_vector: Knot, iterations: usize) -> Vec<Knot> {
        let length = self.knots.len().clone();
        let head_vector = start_vector.clone();
        let mut tail_positions: Vec<Knot> = Vec::new();
        for _ in 0..iterations {
            self.knots[0].move_pt(&head_vector);
            for j in 0..(length - 1) {
                let head = self.knots[j].clone();
                self.knots[j+1].tail_catch_up(&head);
            }
            tail_positions.push(self.knots.last().unwrap().clone());
        }
        return tail_positions
    }
}

fn main() {
    let pt_vecs = HashMap::from([
        ("U", &Knot {x: 0, y: 1}),
        ("R", &Knot {x: 1, y: 0}),
        ("D", &Knot {x: 0, y: -1}),
        ("L", &Knot {x: -1, y: 0})
    ]);
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_input_vec(opts.input_file.clone());
    let mut rope = Rope{knots: Vec::new()};
    rope.knots.push(Knot{x: 0, y: 0}); // head
    rope.knots.push(Knot{x: 0, y: 0}); // tail
    let mut tail_positions: Vec<Knot> = Vec::new();
    tail_positions.push(Knot{x: 0, y: 0});
    for line in lines {
        let mut tokens = line.splitn(2, " ");
        let (direction, amount) = (
            tokens.next().unwrap().to_string(),
            tokens.next().unwrap().to_string().parse::<usize>().unwrap(),
        );
        let pt_vec = pt_vecs.get(direction.as_str()).unwrap();
        let mut new_positions = rope.update(**pt_vec, amount);
        tail_positions.append(&mut new_positions);
    }
    let answer = tail_positions.into_iter().collect::<HashSet<_>>().len();
    println!("Number of positions that tail has visited: {answer}");
}
