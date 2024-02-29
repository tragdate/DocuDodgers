use crate::r#struct::Root;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, ResetColor, SetForegroundColor},
};
use flate2::read::GzDecoder;
use libc::winsize;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{stdout, BufReader, Read, Write},
};

pub fn pick_color(i: u8) -> (u8, u8, u8) {
    let r = (((i * 3) as f32).sin() * 87.0 + 168.0) as u8;
    let g = (((i * 3 + 2) as f32).sin() * 87.0 + 168.0) as u8;
    let b = (((i * 3 + 4) as f32).sin() * 87.0 + 168.0) as u8;
    (r, g, b)
}

fn human_readable_number(num: i64) -> String {
    match num {
        n if n >= 1_000_000 => format!("{:.1}M", n as f64 / 1_000_000f64),
        n if n >= 1000 => format!("{:.1}K", n as f64 / 1000f64),
        _ => num.to_string(),
    }
}

pub fn draw_chart(
    cumulative_sums: &BTreeMap<(i64, i64), BTreeMap<String, i64>>,
    lang_colors: &HashMap<String, Color>,
    lang_positions: &HashMap<String, usize>,
    year: i64,
    month: i64,
) {
    let mut stdout = stdout();
    let data = &cumulative_sums[&(year, month)];
    let (max_questions, max_lang_length) = get_max_values(cumulative_sums);
    for (language, line_number) in lang_positions {
        let questions = data.get(language).unwrap_or(&0);
        let available_width = get_terminal_width() - max_lang_length - 10;
        let bar_length = available_width * *questions as usize / max_questions;
        let color = lang_colors.get(language).unwrap_or(&Color::White);
        execute!(stdout, MoveTo(0, *line_number as u16), SetForegroundColor(*color)).unwrap();
        if bar_length == 0 && *questions > 0 {
            write!(stdout, "{:>width$}: ▌ ({})", language, human_readable_number(*questions), width = max_lang_length).unwrap();
        } else {
            write!(
                stdout,
                "{:>width$}: {} ({})",
                language,
                "█".repeat(bar_length),
                human_readable_number(*questions),
                width = max_lang_length
            )
            .unwrap();
        }
        execute!(stdout, ResetColor).unwrap();
    }
    execute!(stdout, MoveTo(0, lang_positions.len() as u16)).unwrap();
    writeln!(stdout, "\nYear: {}, Month: {:02}", year, month).unwrap();
}

pub fn get_max_values(cumulative_sums: &BTreeMap<(i64, i64), BTreeMap<String, i64>>) -> (usize, usize) {
    cumulative_sums.iter().fold((0, 0), |(max_questions, max_length), (_, language_data)| {
        language_data
            .iter()
            .fold((max_questions, max_length), |(max_questions, max_length), (language, questions)| {
                (max_questions.max(*questions as usize), max_length.max(language.len()))
            })
    })
}

pub fn decompress(body: &[u8]) -> Vec<u8> {
    let mut decompressed_response = Vec::new();
    GzDecoder::new(body).read_to_end(&mut decompressed_response).unwrap();
    decompressed_response
}

pub fn get_data_local(file: &str) -> Root {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub fn get_terminal_width() -> usize {
    term_width().unwrap_or(79)
}

pub fn term_width() -> Option<usize> {
    unsafe {
        let mut size: winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size) != 0 {
            return None;
        }
        Some(size.ws_col as usize - 1)
    }
}

pub fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    std::io::stdout().flush().unwrap();
}
