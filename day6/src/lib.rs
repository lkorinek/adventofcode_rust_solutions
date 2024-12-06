use std::fmt;
use std::ops::Add;

#[derive(PartialEq, Clone)]
pub enum MapObject {Guard, Obstacle, Empty, Visited}

impl fmt::Display for MapObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            MapObject::Guard => '^',
            MapObject::Obstacle => '#',
            MapObject::Empty => '.',
            MapObject::Visited => 'X',
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
pub enum Dir {Left, Right, Down, Up}

impl Dir {
    fn rotate(&self) -> Self {
        match self {
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
        }
    }

    pub fn get_change(&self) -> Pos {
         match self {
             Dir::Left => Pos(0, -1),
             Dir::Right => Pos(0, 1),
             Dir::Up => Pos(-1, 0),
             Dir::Down => Pos(1, 0),
         }
    }
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
pub struct Pos(pub isize, pub isize);

impl Add<Pos> for &Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
pub struct Guard {
    pub pos: Pos,
    pub dir: Dir,
}

impl Guard {
    pub fn rotate(&mut self){
        self.dir = self.dir.rotate();
    }
}
