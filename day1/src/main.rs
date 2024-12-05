use std::{
    fs,
    ops::{Mul, Sub},
};

fn main() {
    let input_string = fs::read_to_string("jdd/input.txt").expect("Impossible de lire le JDD");
    let input_data: Vec<Vec<i32>> = input_string
        .lines()
        .map(|ligne| {
            ligne
                .split("   ")
                .map(String::from)
                .map(|location_id| location_id.parse::<i32>())
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    let mut list_group_1 = input_data
        .iter()
        .map(|couple| couple[0])
        .collect::<Vec<i32>>();
    list_group_1.sort();
    let mut list_group_2 = input_data
        .iter()
        .map(|couple| couple[1])
        .collect::<Vec<i32>>();
    list_group_2.sort();

    let total_distance = list_group_1
        .iter()
        .enumerate()
        .map(|(i, location_id)| location_id.sub(list_group_2[i]).abs())
        .sum::<i32>();

    let similarity_score = list_group_1
        .iter()
        .map(|location_id| {
            (list_group_2
                .iter()
                .filter(|location_id_2| location_id_2.eq(&location_id))
                .count() as i32)
                .mul(location_id)
        })
        .sum::<i32>();

    println!("{}", &total_distance);
    println!("{}", &similarity_score);
}
