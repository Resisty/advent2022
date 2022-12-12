use std::{fmt, collections::HashMap};

#[derive(Clone,Copy,Default,Debug,Hash)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn from_pt(p: &Point) -> Coord {
        Coord { x: p.x.clone(), y: p.y.clone() }
    }
    fn up(&self) -> Coord {
        Coord { x: self.x, y: self.y - 1}
    }
    fn down(&self) -> Coord {
        Coord { x: self.x, y: self.y + 1}
    }
    fn left(&self) -> Coord {
        Coord { x: self.x - 1, y: self.y}
    }
    fn right(&self) -> Coord {
        Coord { x: self.x + 1, y: self.y}
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y 
    }
}
impl Eq for Coord {}

#[derive(Clone,Default,Debug,Eq,PartialEq)]
pub enum PointKind {
    Start,
    #[default]
    Path,
    End
}
#[derive(Clone,Default,Debug)]
struct Point {
    kind: PointKind,
    visited: bool,
    distance: i32,
    x: i32,
    y: i32,
    height: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) height: {}, distance: {}", self.x, self.y, self.height, self.distance)
    }
}

fn main() {
    let opts = adventils::OptionalArgumentsAndSuchLike::args();
    // implement solutions in here somewhere
    let lines = adventils::get_input_vec(opts.input_file.clone());
    let mut points: HashMap<Coord,Point> = HashMap::new();
    let (mut height, mut width) = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            let kind = match chr {
                'S' => PointKind::Start,
                'E' => PointKind::End,
                _ => PointKind::Path
            };
            let (height, distance) = match kind {
                PointKind::Start => ('a' as i32, 0),
                PointKind::End => ('z' as i32, i32::MAX),
                _ => (chr as i32, i32::MAX)
            };
            let c = Coord{x: x as i32, y: y as i32};
            let pt = Point {
                kind,
                visited: false,
                distance,
                x: x as i32,
                y: y as i32,
                height,
            };
            points.insert(c, pt);
            width = x + 1;
        }
        height = y  + 1;
    }
    let tmp = points.iter().filter(|(_, p)| p.kind == PointKind::Start).map(|(_, p)| (*p).clone()).collect::<Vec<Point>>();
    let mut current = tmp.first().unwrap().clone();
    let mut neighbors = get_unvisited_neighbors(&current, &points);
    loop {
        for neighbor in &neighbors {
            let neighbor_pt = points.get_mut(&neighbor).unwrap();
            if neighbor_pt.distance > current.distance + 1 {
                if neighbor_pt.height <= current.height + 1 {
                    neighbor_pt.distance = current.distance + 1;
                }
            }
        }
        current.visited = true;
        points.remove(&Coord::from_pt(&current));
        if current.kind == PointKind::End {
            println!("Reach the end");
            break // Nowhere else to go
        }
        let shortest = points.iter().min_by(|a, b| a.1.distance.cmp(&b.1.distance)).map(|(_, p)| p);
        if shortest.is_none() || shortest.unwrap().distance == i32::MAX {
            println!("Shortest is None or minimum distance is i32::MAX");
            break // Either there are no more points in the set or the shortest distance is INF
        }
        println!("Shortest route on map is {}", shortest.unwrap());
        current = shortest.unwrap().clone();
        println!("---");
        printit(&points, width as i32, height as i32);
        neighbors = get_unvisited_neighbors(&current, &points);
    }
    let answer = current.distance;
    println!("Shortest path length: {answer}", );
}

fn printit(points: &HashMap<Coord,Point>, w: i32, h: i32) {
    for y in 0..h {
        let mut s = String::from("");
        for x in 0..w {
            match points.get(&Coord{x, y}) {
                Some(p) => {
                    if p.visited {
                        s.push_str("#");
                    } else {
                        s.push_str(".");
                    }
                },
                None => {
                    s.push_str("#");
                }
            }
        }
        println!("{s}");
    }
}

fn get_unvisited_neighbors(current: &Point, points: &HashMap<Coord,Point>) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    let (u, r, d, l) = (Coord::from_pt(current).up(), Coord::from_pt(current).right(), Coord::from_pt(current).down(), Coord::from_pt(current).left());
    for c in [u, r, d, l] {
        match points.get(&c) {
            None => (), // off the map
            Some(pt) => {
                match pt.visited {
                    true => (), // do not consider visted points
                    false => {
                        // make sure it's climbable
                        if pt.height <= current.height + 1 {
                            coords.push(c)
                        }
                    }
                }
            }
        }
    }
    coords
}
