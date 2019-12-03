use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use crate::coord::{Coord, Direction, Path};

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(anyhow!(
                "String must be at least two characters, got {}",
                s.len()
            ));
        }
        let distance_res = s[1..].parse::<usize>();
        Ok(match &s[0..1] {
            "U" => Direction::Up(distance_res?),
            "R" => Direction::Right(distance_res?),
            "D" => Direction::Down(distance_res?),
            "L" => Direction::Left(distance_res?),
            c => return Err(anyhow!("Unexpected direction {:?}", c)),
        })
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Path, Self::Err> {
        Ok(s.split(",")
            .map(|dir| Ok(dir.parse::<Direction>()?))
            .collect::<Result<Vec<_>>>()?
            .into())
    }
}

fn add_steps((i, coord): (usize, Coord)) -> (Coord, usize) {
    (coord, i + 1)
}

fn solve(wire_a: Path, wire_b: Path) -> (usize, Option<usize>) {
    let wire_a_steps: HashMap<_, _> = wire_a.walk().enumerate().map(add_steps).collect();

    let mut intersect_coords = Vec::new();
    let mut intersect_steps = Vec::new();
    for (coord, num_steps_b) in wire_b.walk().enumerate().map(add_steps) {
        if let Some(num_steps_a) = wire_a_steps.get(&coord) {
            intersect_coords.push(coord);
            intersect_steps.push(num_steps_a + num_steps_b);
        }
    }

    (
        intersect_coords
            .iter()
            .map(|&coord| coord.distance_from_origin())
            .min()
            .unwrap(),
        intersect_steps.into_iter().min(),
    )
}

pub fn main(args: &[String]) -> Result<(usize, Option<usize>)> {
    let file = File::open(&args[0])?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let wire_a = lines.next().unwrap()?.parse::<Path>()?;
    let wire_b = lines.next().unwrap()?.parse::<Path>()?;

    Ok(solve(wire_a, wire_b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_direction() -> Result<()> {
        assert_eq!("U123".parse::<Direction>()?, Direction::Up(123));
        assert_eq!("R456".parse::<Direction>()?, Direction::Right(456));
        assert_eq!("D789".parse::<Direction>()?, Direction::Down(789));
        assert_eq!("L1".parse::<Direction>()?, Direction::Left(1));
        Ok(())
    }

    #[test]
    fn test_parse_path() -> Result<()> {
        assert_eq!(
            "U1,D2,R10".parse::<Path>()?,
            Path::new(vec![
                Direction::Up(1),
                Direction::Down(2),
                Direction::Right(10)
            ])
        );
        Ok(())
    }

    #[test]
    fn test_solve() -> Result<()> {
        assert_eq!(
            solve("R8,U5,L5,D3".parse()?, "U7,R6,D4,L4".parse()?),
            (6, Some(30))
        );
        assert_eq!(
            solve(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72".parse()?,
                "U62,R66,U55,R34,D71,R55,D58,R83".parse()?
            ),
            (159, Some(610))
        );
        assert_eq!(
            solve(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".parse()?,
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".parse()?
            ),
            (135, Some(410))
        );
        Ok(())
    }
}
