use std::fmt;

#[derive(Default,Debug)]
enum Operator {
    #[default]
    Add,
    Mult
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, " + "),
            Operator::Mult => write!(f, " * "),
        }
    }
}

#[derive(Default,Debug)]
enum Operand {
    #[default]
    Old,
    Number(i64)
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Old => write!(f, "old"),
            Operand::Number(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Default,Debug)]
struct Operation {
    operator: Operator,
    operand: Operand
}

impl Operation {
    fn doit(&self, item: i64, max: i64) -> i64 {
        let roper = match self.operand {
            Operand::Old => item,
            Operand::Number(n) => n
        };
        match self.operator {
            Operator::Add => item + roper,
            Operator::Mult => item * roper % max
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.operator, self.operand)
    }
}

#[derive(Default,Debug)]
pub struct Monkey {
    id: i64,
    items: Vec<i64>,
    operation: Operation,
    test_divisor: i64,
    test_t: i64,
    test_f: i64,
    num_inspects: i64
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(items: {:?}, operation: {}, test_divisor: {}, test_true: {}, test_false: {})", self.items, self.operation, self.test_divisor, self.test_t, self.test_f)
    }
}

impl Monkey {
    pub fn turn(&mut self, max: i64) -> Vec<(i64, i64)>{
        let mut item_to_next: Vec<(i64, i64)> = Vec::new();
        self.num_inspects += self.items.len().clone() as i64;
        while self.items.len() > 0 {
            let mut item = self.items.remove(0);
            item = self.operation.doit(item, max);
            let next_monkey_id = if item % self.test_divisor == 0 {self.test_t} else {self.test_f};
            item_to_next.push((item, next_monkey_id));
        }
        item_to_next
    }
}

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_input(opts.input_file.clone());
    let monkeys = lines.split("\n\n");
    let mut monkey_vec: Vec<Monkey> = Vec::new();
    for monkey_desc in monkeys {
        let monkey = parse_monkey(monkey_desc);
        monkey_vec.push(monkey);
    }
    let max: i64 = monkey_vec.iter().map(|m| m.test_divisor).product();
    for _ in 0..10000 {
        let num_monkeys = monkey_vec.len().clone();
        for i in 0..num_monkeys {
            // Fuckin' hell this language is so stupid
            let the_monkey = monkey_vec.get_mut(i as usize).unwrap();
            let items_to_next = the_monkey.turn(max);
            for (item, monkey_id) in items_to_next {
                monkey_vec.get_mut(monkey_id as usize).unwrap().items.push(item);
            }
        }
    }
    monkey_vec.sort_by_key(|k| k.num_inspects);
    let answer = monkey_vec.iter().map(|m| m.num_inspects).rev().take(2).collect::<Vec<i64>>();
    println!("Monkey business: {}", answer[0] * answer[1]);
}

fn parse_monkey(instruction: &str) -> Monkey {
    let mut monkey = Monkey::default();
    for line in instruction.lines().map(|l| l.trim()).collect::<Vec<&str>>() {
        match line.trim() {
            _ if line.starts_with("Monkey") => monkey.id = line.split(" ").last().unwrap().replace(":", "").parse().unwrap(),
            _ if line.starts_with("Starting items") => {
                let split: Vec<&str> = line.split(": ").collect();
                monkey.items = split[1].split(", ").map(|i| i.parse().unwrap()).collect::<Vec<i64>>();
            },
            _ if line.starts_with("Operation") => {
                let operator = match line {
                    _ if line.contains("+") => Operator::Add,
                    _ if line.contains("*") => Operator::Mult,
                    _ => panic!("Line {line} contains no known operator")
                };
                let roperand = line.split(" ").last().unwrap();
                let operand = match roperand {
                    _ if roperand == "old" => Operand::Old,
                    _ => Operand::Number(roperand.parse().unwrap())
                };
                monkey.operation = Operation{ operator, operand };
            },
            _ if line.starts_with("Test") => {
                monkey.test_divisor = line.split(" ").last().unwrap().parse().unwrap();
            },
            _ if line.starts_with("If true") => monkey.test_t = line.split(" ").last().unwrap().parse().unwrap(),
            _ if line.starts_with("If false") => monkey.test_f = line.split(" ").last().unwrap().parse().unwrap(),
            _  => ()
        }
    }
    return monkey;
}
