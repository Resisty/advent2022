#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Throws {
    Rock,
    Paper,
    Scissors
}

pub trait Beats {
    fn beats(&self) -> Self;
}

pub trait LosesTo {
    fn loses_to(&self) -> Self;
}
pub trait Ties {
    fn ties(&self) -> Self;
}

impl Beats for Throws {
    fn beats(&self) -> Self {
        match *self {
            Throws::Rock => Throws::Scissors,
            Throws::Paper => Throws::Rock,
            Throws::Scissors => Throws::Paper
        }
    }
}

impl LosesTo for Throws {
    fn loses_to(&self) -> Self {
        match *self {
            Throws::Rock => Throws::Paper,
            Throws::Paper => Throws::Scissors,
            Throws::Scissors => Throws::Rock
        }
    }
}

impl Ties for Throws {
    fn ties(&self) -> Self {
        match *self {
            Throws::Rock => Throws::Rock,
            Throws::Paper => Throws::Paper,
            Throws::Scissors => Throws::Scissors
        }
    }
}

fn points_for(throw: Throws) -> i32 {
    match throw {
        Throws::Rock => 1,
        Throws::Paper => 2,
        Throws::Scissors => 3
    }
}

fn play(mine: Throws, other: Throws) -> i32 {
    let (mine_beats, other_beats) = (mine.beats(), other.beats());
    let round_bonus = match (mine_beats, other_beats) {
        _ if mine_beats == other => 6,
        _ if other_beats == mine => 0,
        _ => 3
    };
    return round_bonus + points_for(mine)
}

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let vec_lines = adventils::get_input_vec(opts.input_file.clone());
    println!("Got {} rounds to tabulate", vec_lines.len());
    let points: i32 = vec_lines.iter().map(|l| score_round(&l)).sum();
    println!("Points: {points}");
}

fn score_round(line: &String) -> i32 {
    println!("Checking line: {line}");
    let mut tokens = line.split(' ');
    let left = tokens.next().unwrap();
    let right = tokens.next().unwrap();
    // matching stringlikes has to be one of the most painful things I've ever done
    // Whoever designed this language is a sadist
    // Theoretically there's a way to match both at once in a clever way but it probably takes
    // blood sacrifice
    let opponent = match left {
        _ if left == "A" => Throws::Rock,
        _ if left == "B" => Throws::Paper,
        _ if left == "C" => Throws::Scissors,
        _ => {
            panic!("Oh fuck");
        }
    };
    let mine = match right {
        _ if right == "X" => opponent.beats(),
        _ if right == "Y" => opponent.ties(),
        _ if right == "Z" => opponent.loses_to(),
        _ => {
            panic!("Goddammit");
        }
    };
    
    return play(mine, opponent)
}
