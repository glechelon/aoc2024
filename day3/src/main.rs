use std::{fs, ops::Mul};

use regex::Regex;

fn main() {
    let input_string = fs::read_to_string("jdd/input.txt").expect("Impossible de lire le JDD");
    let regex_mul =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Problème avec la définition de la regex");
    let somme = regex_mul
        .captures_iter(&input_string)
        .map(|params| {
            dbg!(&params);
            params[1].parse::<i32>().unwrap() * params[2].parse::<i32>().unwrap()
        })
        .sum::<i32>();

    dbg!(&somme);
}
