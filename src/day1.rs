use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_fuel_req(mass: usize) -> usize {
    let mass = mass / 3;
    if mass < 2 {
        0
    } else {
        mass - 2
    }
}

fn get_fuel_load_req(mass: usize) -> usize {
    let mut total = 0;
    let mut current_mass = mass;
    while current_mass != 0 {
        current_mass = get_fuel_req(current_mass);
        total += current_mass;
    }
    total
}

pub fn main(args: &[String]) -> Result<(usize, Option<usize>)> {
    if args.len() != 1 {
        return Err(anyhow!("Expected path to input"));
    }

    let file = File::open(&args[0])?;
    let reader = BufReader::new(file);

    let mut sum_a = 0;
    let mut sum_b = 0;
    for mass in reader.lines().map(|x| x.unwrap().parse::<usize>().unwrap()) {
        sum_a += get_fuel_req(mass);
        sum_b += get_fuel_load_req(mass);
    }

    Ok((sum_a, Some(sum_b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel_req() {
        assert_eq!(get_fuel_req(12), 2);
        assert_eq!(get_fuel_req(14), 2);
        assert_eq!(get_fuel_req(1969), 654);
        assert_eq!(get_fuel_req(100756), 33583);
    }

    #[test]
    fn test_get_fuel_load_req() {
        assert_eq!(get_fuel_load_req(12), 2);
        assert_eq!(get_fuel_load_req(14), 2);
        assert_eq!(get_fuel_load_req(1969), 966);
        assert_eq!(get_fuel_load_req(100756), 50346);
    }
}
