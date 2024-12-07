use std::{fs, ops::Mul};

use regex::Regex;

fn main() {
    let input_string = fs::read_to_string("jdd/input.txt")
        .expect("Impossible de lire le JDD")
        .replace("\n", "")
        .replace(" ", "");
    dbg!(&input_string.replace("\n", ""));
    let regex_mul =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Problème avec la définition de la regex");

    let regex = Regex::new(r"^.*?don't\(\)|do\(\).*?(don't\(\)|$)")
        .expect("Problème avec la la définition de la regex");

    let somme = compute(&regex_mul, &input_string);

    let somme_do_dont = regex
        .find_iter(&input_string)
        .map(|bloc| compute(&regex_mul, &bloc.as_str().to_string()))
        .sum::<i32>();

    dbg!(&somme);
    dbg!(&somme_do_dont);

    //  680910
    // 423950

    //211975
    //423950
    //5789508
    //5002453
    //468935
}

fn compute(regex_mul: &Regex, input_string: &String) -> i32 {
    regex_mul
        .captures_iter(input_string)
        .map(|params| params[1].parse::<i32>().unwrap() * params[2].parse::<i32>().unwrap())
        .sum::<i32>()
}
