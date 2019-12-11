use anyhow::{anyhow, Result};

use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn checksum(layer_size: usize, digits: &[u8]) -> Result<usize> {
    if digits.len() % layer_size != 0 {
        return Err(anyhow!("Layer size must be evenly divisible by input"));
    }
    let fewest_zeroes = digits
        .chunks(layer_size)
        .map(|layer| {
            let mut counts: HashMap<u8, usize> = HashMap::new();
            for digit in layer {
                let e = counts.entry(*digit).or_insert(0);
                *e += 1;
            }
            counts
        })
        .min_by_key(|counts| *counts.get(&0).unwrap_or(&0))
        .ok_or(anyhow!("No digits found"))?;
    Ok(fewest_zeroes.get(&1).unwrap_or(&0) * fewest_zeroes.get(&2).unwrap_or(&0))
}

pub fn render(width: usize, height: usize, digits: &[u8]) -> Result<String> {
    let layer_size = width * height;
    if digits.len() % layer_size != 0 {
        return Err(anyhow!("Layer size must be evenly divisible by input"));
    }

    let mut pixels = Vec::new();
    pixels.resize_with(layer_size, || 2);

    for layer in digits.chunks(layer_size) {
        for (i, color) in layer.iter().enumerate() {
            if pixels[i] == 2 {
                pixels[i] = *color;
            }
        }
    }

    Ok(pixels
        .as_slice()
        .chunks(width)
        .map(|row| {
            row.iter()
                .map(|pixel| match pixel {
                    1 => '#',
                    _ => ' ',
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n"))
}

pub fn main(args: &[String]) -> Result<(usize, Option<String>)> {
    if args.len() != 1 {
        return Err(anyhow!("Expected path to input"));
    }

    let file = File::open(&args[0])?;
    let reader = BufReader::new(file);
    let line = reader
        .lines()
        .next()
        .ok_or(anyhow!("Unable to read file"))??;

    let width = 25;
    let height = 6;

    let digits = line
        .chars()
        .map(|x| match x.to_digit(10) {
            Some(digit) => Ok(digit.try_into()?),
            None => Err(anyhow!("Character is not a base 10 digit")),
        })
        .collect::<Result<Vec<u8>, _>>()?;

    Ok((
        checksum(width * height, digits.as_slice())?,
        Some(render(width, height, digits.as_slice())?),
    ))
}
