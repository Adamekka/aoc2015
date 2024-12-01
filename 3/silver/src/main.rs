use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i8,
    y: i8,
}

impl Pos {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    let mut pos = Pos::new();

    let mut visited = HashSet::<Pos>::new();

    for c in text.chars() {
        match c {
            '^' => pos.up(),
            'v' => pos.down(),
            '>' => pos.right(),
            '<' => pos.left(),
            _ => unreachable!(),
        }

        visited.insert(pos.clone());
    }

    println!("{}", visited.len());
}
