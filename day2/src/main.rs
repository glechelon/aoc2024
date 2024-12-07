use std::{fs, ops::Sub, usize};

fn main() {
    let input_string = fs::read_to_string("jdd/input.txt").expect("Impossible de lire le JDD");
    let input_data = input_string
        .lines()
        .map(|ligne| {
            ligne
                .split(" ")
                .map(String::from)
                .map(|element| element.parse::<i32>())
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let level_deltas_by_report = calculer_deltas(input_data);

    let nombre_de_reports_safe = level_deltas_by_report
        .iter()
        .filter(|level_deltas| is_report_safe(level_deltas.1.clone()))
        .count();

    let analyzed_reports: Vec<(Vec<i32>, Option<Vec<(usize, i32)>>)> = level_deltas_by_report
        .iter()
        .map(|level_deltas| {
            (
                level_deltas.0.clone(),
                analyze_report_for_dampening(level_deltas.1.clone()),
            )
        })
        .collect();

    let nombre_corriges = analyzed_reports
        .iter()
        .filter(|analyzed_report| analyzed_report.1.is_some())
        .map(|analyzed_report| {
            (
                analyzed_report.0.clone(),
                analyzed_report.clone().1.unwrap(),
            )
        })
        .filter(|analyzed_report| !analyzed_report.1.is_empty())
        .map(|analyzed_report| {
            analyzed_report
                .1
                .iter()
                .map(|delta_problematique| {
                    vec![delta_problematique.0, delta_problematique.0 + 1]
                        .iter()
                        .map(|index| {
                            let mut report_corrige = analyzed_report.0.clone();
                            report_corrige.remove(*index);
                            return is_report_safe(calculer_deltas_report(&report_corrige).1);
                        })
                        .any(|safe| safe)
                })
                .any(|safe| safe)
        })
        .filter(|safe| *safe)
        .count();

    dbg!(nombre_de_reports_safe);
    dbg!(nombre_de_reports_safe + nombre_corriges);
}

fn calculer_deltas(input_data: Vec<Vec<i32>>) -> Vec<(Vec<i32>, Vec<i32>)> {
    let level_deltas_by_report = input_data
        .iter()
        .map(|report| calculer_deltas_report(report))
        .collect::<Vec<(Vec<i32>, Vec<i32>)>>();
    level_deltas_by_report
}

fn calculer_deltas_report(report: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    (
        report.clone(),
        report
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, level)| level.sub(report[i - 1]))
            .collect(),
    )
}

fn is_report_safe(level_deltas: Vec<i32>) -> bool {
    let deltas_corrects = level_deltas
        .iter()
        .all(|delta| delta.abs() > 0 && delta.abs() <= 3);
    let tous_augmentent = level_deltas.iter().all(|delta| delta.is_positive());
    let tous_diminuent = level_deltas.iter().all(|delta| delta.is_negative());
    deltas_corrects && (tous_augmentent || tous_diminuent)
}

fn analyze_report_for_dampening(level_deltas: Vec<i32>) -> Option<Vec<(usize, i32)>> {
    dbg!(&level_deltas);
    let deltas_incorrects_avec_postions = level_deltas
        .clone()
        .iter()
        .enumerate()
        .filter(|(_, delta)| !(delta.abs() > 0 && delta.abs() <= 3))
        .map(|(i, delta)| (i.clone(), delta.clone()))
        .collect::<Vec<(usize, i32)>>();
    // dbg!(&deltas_incorrects_avec_postions);
    let deltas_positifs: Vec<(usize, i32)> = level_deltas
        .clone()
        .iter()
        .enumerate()
        .filter(|(_, delta)| delta.is_positive())
        .map(|(i, delta)| (i.clone(), delta.clone()))
        .collect();
    // dbg!(&deltas_positifs);
    let deltas_negatifs: Vec<(usize, i32)> = level_deltas
        .clone()
        .iter()
        .enumerate()
        .filter(|(_, delta)| delta.is_negative())
        .map(|(i, delta)| (i.clone(), delta.clone()))
        .collect();
    //dbg!(&deltas_negatifs);
    if deltas_incorrects_avec_postions.is_empty()
        && (deltas_positifs.is_empty() || deltas_negatifs.is_empty())
    {
        return None;
    } else if deltas_incorrects_avec_postions.len() == 1
        || deltas_positifs.len() == 1 && deltas_negatifs.len() > deltas_positifs.len()
        || deltas_negatifs.len() == 1 && deltas_positifs.len() > deltas_negatifs.len()
    {
        //dbg!(deltas_positifs.len() == 1);
        let deltas_signe_different = if deltas_positifs.len() == 1 {
            Some(deltas_positifs)
        } else if deltas_negatifs.len() == 1 {
            Some(deltas_negatifs)
        } else {
            None
        };
        dbg!(&deltas_signe_different);

        let mut deltas_a_supprimer = deltas_incorrects_avec_postions;
        if deltas_signe_different.is_some() {
            deltas_a_supprimer.extend(deltas_signe_different.unwrap());
            deltas_a_supprimer.sort();
            deltas_a_supprimer.dedup();
        }
        return Some(deltas_a_supprimer);
    } else {
        dbg!(&level_deltas);
        return Some(Vec::new());
    }
}
