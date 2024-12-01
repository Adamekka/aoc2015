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

    fn move_to(&mut self, c: char, visited: &mut HashSet<Pos>) {
        match c {
            '^' => self.up(),
            'v' => self.down(),
            '>' => self.right(),
            '<' => self.left(),
            _ => unreachable!(),
        }

        visited.insert(self.clone());
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").unwrap();

    let mut santa = Pos::new();
    let mut robo_santa = Pos::new();

    let mut visited = HashSet::<Pos>::new();

    let mut santa_moves = true;

    for c in text.chars() {
        if santa_moves {
            santa.move_to(c, &mut visited);
        } else {
            robo_santa.move_to(c, &mut visited);
        }

        santa_moves = !santa_moves;
    }

    println!("{}", visited.len());
}
