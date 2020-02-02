#![allow(dead_code)]

trait Battlecry {
    fn bc(&self, f: dyn Fn(Board) -> Board);
}

trait Deathrattle {
    fn dr(&self, f: dyn Fn(Board) -> Board);
}

struct Hero {
    name: String,
    health: i32,
}

struct Card {
    f: Option<Box<dyn Fn(&mut Board)>>,
    battlecry: Option<Box<dyn Battlecry>>,
    deathrattle: Option<Box<dyn Deathrattle>>
}

struct Board {
    top: Vec<Card>,
    bottom: Vec<Card>
}

fn main() {
    println!("Hello, world!");
}
