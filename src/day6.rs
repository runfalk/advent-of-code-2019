use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result as StdResult;

type Orbits = HashMap<String, String>;

struct ParentIterator<'a> {
    orbits: &'a Orbits,
    curr: &'a str,
}

impl<'a> ParentIterator<'a> {
    pub fn new(orbits: &'a Orbits, start: &'a str) -> Self {
        Self { orbits, curr: start }
    }
}

impl<'a> Iterator for ParentIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let parent = self.orbits.get(self.curr)?;
        self.curr = parent.as_str();
        Some(self.curr)
    }
}

fn find_min_num_transfers(orbits: &Orbits, a: &str, b: &str) -> Result<usize> {
    let a_parents: HashMap<_, _> = ParentIterator::new(orbits, a)
        .enumerate()
        .map(|(dist, parent)| (parent, dist))
        .collect();

    let b_parent_iter = ParentIterator::new(orbits, b)
        .enumerate()
        .map(|(dist, parent)| (parent, dist));
    for (b_parent, b_dist) in b_parent_iter {
        if let Some(a_dist) = a_parents.get(&b_parent) {
            return Ok(a_dist + b_dist);
        }
    }

    Err(anyhow!(""))
}

pub fn main(args: &[String]) -> Result<(usize, Option<usize>)> {
    if args.len() != 1 {
        return Err(anyhow!("Expected path to input"));
    }

    let file = File::open(&args[0])?;
    let reader = BufReader::new(file);

    let orbits = reader
        .lines()
        .map(|orbit_str| -> Result<(String, String)> {
            let orbit_str = orbit_str?;
            let mut mass_iter = orbit_str.split(")");

            let mass = mass_iter.next().unwrap();
            let satellite = mass_iter.next().ok_or(anyhow!("No orbit separator found"))?;
            Ok((satellite.into(), mass.into()))
        })
        .collect::<StdResult<Orbits, _>>()?;

    let total_num_orbits: usize = orbits
        .keys()
        .map(|planet| ParentIterator::new(&orbits, planet.as_str()).count())
        .sum();

    Ok((total_num_orbits, Some(find_min_num_transfers(&orbits, "YOU", "SAN")?)))
}
