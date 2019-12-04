use std::ops::{Add, Sub};

use self::Direction::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    dirs: Vec<Direction>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathIterator {
    origin: Coord,
    path: Path,
    curr_dir: usize,
    i: usize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }

    pub fn origin() -> Self {
        Coord { x: 0, y: 0 }
    }

    pub fn distance(a: Self, b: Self) -> usize {
        let relative_coord = a - b;
        (relative_coord.x.abs() + relative_coord.y.abs()) as usize
    }

    pub fn distance_from_origin(&self) -> usize {
        Self::distance(*self, Self::origin())
    }

    pub fn offset(&self, dir: Direction) -> Self {
        *self
            + match dir {
                Up(i) => Coord::new(0, i as isize),
                Right(i) => Coord::new(i as isize, 0),
                Down(i) => Coord::new(0, -(i as isize)),
                Left(i) => Coord::new(-(i as isize), 0),
            }
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Direction {
    pub fn resize(&self, len: usize) -> Direction {
        match self {
            Up(_) => Up(len),
            Right(_) => Right(len),
            Down(_) => Down(len),
            Left(_) => Left(len),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Up(n) => *n,
            Right(n) => *n,
            Down(n) => *n,
            Left(n) => *n,
        }
    }
}

impl Path {
    pub fn new(dirs: Vec<Direction>) -> Self {
        Self { dirs }
    }

    pub fn walk(self) -> PathIterator {
        self.walk_from(Coord::origin())
    }

    pub fn walk_from(self, origin: Coord) -> PathIterator {
        PathIterator::new(origin, self)
    }
}

impl From<Direction> for Path {
    fn from(dir: Direction) -> Self {
        Path::new(vec![dir])
    }
}

impl From<Vec<Direction>> for Path {
    fn from(dirs: Vec<Direction>) -> Self {
        Path::new(dirs)
    }
}

impl PathIterator {
    pub fn new(origin: Coord, path: Path) -> Self {
        PathIterator {
            origin,
            path,
            curr_dir: 0,
            i: 0,
        }
    }
}

impl Iterator for PathIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_dir == self.path.dirs.len() {
            return None;
        }

        let dir = self.path.dirs[self.curr_dir];
        let output = if self.i < dir.len() {
            self.i += 1;
            Some(self.origin.offset(dir.resize(self.i)))
        } else {
            None
        };

        if self.i == dir.len() {
            self.origin = self.origin.offset(dir);
            self.curr_dir += 1;
            self.i = 0;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Coord::new(1, 3) + Coord::new(2, 4), Coord::new(3, 7));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Coord::new(1, 3) - Coord::new(2, 4), Coord::new(-1, -1));
    }

    #[test]
    fn test_distance() {
        let a = Coord::new(3, 4);
        let b = Coord::new(1, 1);

        assert_eq!(a.distance_from_origin(), 7);
        assert_eq!(b.distance_from_origin(), 2);

        assert_eq!(Coord::distance(a, b), 5);
    }

    #[test]
    fn test_offset() {
        assert_eq!(Coord::origin().offset(Up(100)), Coord::new(0, 100));
        assert_eq!(Coord::origin().offset(Right(100)), Coord::new(100, 0));
        assert_eq!(Coord::origin().offset(Down(100)), Coord::new(0, -100));
        assert_eq!(Coord::origin().offset(Left(100)), Coord::new(-100, 0));
    }

    #[test]
    fn test_walk() {
        assert_eq!(Path::from(Up(1000)).walk().count(), 1000);
        assert_eq!(
            Path::from(Up(1000)).walk().last().unwrap(),
            Coord::new(0, 1000)
        );

        assert_eq!(
            Path::new(vec![Up(1), Left(1), Down(1), Right(1)])
                .walk()
                .count(),
            4
        );
        assert_eq!(
            Path::new(vec![Up(1), Left(1), Down(1), Right(1)])
                .walk()
                .last()
                .unwrap(),
            Coord::origin()
        );
        assert_eq!(
            Path::new(vec![Up(10), Left(10), Down(5), Right(5)])
                .walk()
                .last()
                .unwrap(),
            Coord::new(-5, 5)
        );
    }
}
