#![allow(non_snake_case)]
mod request;
mod r#struct;
mod utils;
use crossterm::style::Color;
use request::get_data;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::thread;
use std::time::Duration;
use structopt::StructOpt;
use utils::{clear_terminal, draw_chart, get_data_local, pick_color};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, required_unless = "remote")]
    local: Option<String>,
    #[structopt(long, required_unless = "local")]
    remote: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    let root = if opt.local.is_some() {
        get_data_local(&opt.local.unwrap())
    } else if opt.remote.is_some() {
        get_data(&opt.remote.unwrap())
    } else {
        eprintln!("At least one of --local or --remote must be provided.");
        std::process::exit(1);
    };
    clear_terminal();
    let mut dataset: BTreeMap<(i64, i64), Vec<(String, i64)>> = BTreeMap::new();
    for result_set in &root.result_sets {
        for row in &result_set.rows {
            let (year, month, language, questions) = row;
            dataset
                .entry((*year, *month))
                .or_default()
                .push((language.clone(), *questions));
        }
    }
    let mut cumulative_sums: BTreeMap<(i64, i64), BTreeMap<String, i64>> = BTreeMap::new();
    let mut lang_colors: HashMap<String, Color> = HashMap::new();
    let mut i = 0;
    for ((year, month), data) in &dataset {
        let mut month_data = cumulative_sums
            .get(&(*year, *month - 1))
            .cloned()
            .unwrap_or_default();

        if *month == 1 && *year != dataset.keys().next().unwrap().0 {
            for (language, questions) in cumulative_sums.get(&(*year - 1, 12)).unwrap() {
                *month_data.entry(language.to_string()).or_default() += questions;
                let (r, g, b) = pick_color(i * 2);
                lang_colors.insert(language.clone(), Color::Rgb { r, g, b });
                i += 1;
            }
        }
        for (language, questions) in data {
            *month_data.entry(language.to_string()).or_default() += questions;
            if !lang_colors.contains_key(language) {
                let (r, g, b) = pick_color(i * 3);
                lang_colors.insert(language.clone(), Color::Rgb { r, g, b });
                i += 1;
            }
        }
        cumulative_sums.insert((*year, *month), month_data);
    }
    let mut lang_positions: HashMap<String, usize> = HashMap::new();
    let mut line_number = 0;

    let mut unique_languages: HashSet<String> = HashSet::new();
    for language_data in cumulative_sums.values() {
        for language in language_data.keys() {
            unique_languages.insert(language.clone());
        }
    }
    for language in unique_languages {
        if !lang_positions.contains_key(&language) {
            lang_positions.insert(language, line_number);
            line_number += 1;
        }
    }

    for ((year, month), language_data) in &cumulative_sums {
        let mut language_data_vec: Vec<(String, i64)> =
            language_data.iter().map(|(k, v)| (k.clone(), *v)).collect();
        language_data_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort the vector of language data
        for (i, (language, _)) in language_data_vec.iter().enumerate() {
            lang_positions.insert(language.clone(), i); // Assign new line positions
        }
        let _ = draw_chart(
            &cumulative_sums,
            &lang_colors,
            &lang_positions,
            *year,
            *month,
        );
        thread::sleep(Duration::from_millis(100));
    }
    //press anything to exit
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
