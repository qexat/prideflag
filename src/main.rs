use std::{collections::HashMap, process};

use clap::Parser;
use lazy_static::lazy_static;

const DEFAULT_FLAG: &'static str = "pride";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    flag: Option<String>,
}

impl Args {
    fn get_flag_name(self) -> String {
        match self.flag {
            Some(v) => v.to_lowercase(),
            None => DEFAULT_FLAG.to_owned(),
        }
    }
}

mod colors {
    pub type RGBColor = (u8, u8, u8);

    pub const WHITE: RGBColor = (255, 255, 255);
    pub const RED: RGBColor = (228, 3, 3);
    pub const SINOPIA: RGBColor = (214, 41, 0);
    pub const ORANGE: RGBColor = (255, 140, 0);
    pub const ATOMIC_TANGERINE: RGBColor = (255, 155, 85);
    pub const YELLOW: RGBColor = (255, 237, 0);
    pub const GREEN: RGBColor = (0, 128, 38);
    pub const MINT: RGBColor = (152, 232, 193);
    pub const TURQUOISE: RGBColor = (38, 206, 170);
    pub const TEAL: RGBColor = (7, 141, 112);
    pub const MAYA_BLUE: RGBColor = (85, 205, 252);
    pub const COBALT_BLUE: RGBColor = (123, 173, 226);
    pub const MAJORELLE_BLUE: RGBColor = (80, 73, 204);
    pub const BLUE: RGBColor = (0, 56, 168);
    pub const INDIGO: RGBColor = (36, 64, 142);
    pub const PURPLE: RGBColor = (61, 26, 120);
    pub const MAUVE: RGBColor = (155, 79, 150);
    pub const AMARANTH_PINK: RGBColor = (247, 168, 184);
    pub const SUPER_PINK: RGBColor = (212, 97, 166);
    pub const RUBY: RGBColor = (214, 2, 112);
    pub const FLIRT: RGBColor = (165, 0, 98);
    pub const VIOLET: RGBColor = (115, 41, 130);
}

lazy_static! {
    static ref FLAGS: HashMap<&'static str, Vec<colors::RGBColor>> = {
        let mut h = HashMap::new();
        h.insert(
            "pride",
            vec![
                colors::RED,
                colors::ORANGE,
                colors::YELLOW,
                colors::GREEN,
                colors::INDIGO,
                colors::VIOLET,
            ],
        );
        h.insert(
            "lesbian",
            vec![
                colors::SINOPIA,
                colors::ATOMIC_TANGERINE,
                colors::WHITE,
                colors::SUPER_PINK,
                colors::FLIRT,
            ],
        );
        h.insert(
            "gay",
            vec![
                colors::TEAL,
                colors::TURQUOISE,
                colors::MINT,
                colors::WHITE,
                colors::COBALT_BLUE,
                colors::MAJORELLE_BLUE,
                colors::PURPLE,
            ],
        );
        h.insert(
            "bisexual",
            vec![
                colors::RUBY,
                colors::RUBY,
                colors::MAUVE,
                colors::BLUE,
                colors::BLUE,
            ],
        );
        h.insert(
            "transgender",
            vec![
                colors::MAYA_BLUE,
                colors::AMARANTH_PINK,
                colors::WHITE,
                colors::AMARANTH_PINK,
                colors::MAYA_BLUE,
            ],
        );
        h
    };
    static ref FLAG_ALIASES: HashMap<Vec<&'static str>, &'static str> = {
        let mut h = HashMap::new();
        h.insert(vec!["p"], "pride");
        h.insert(vec!["l"], "lesbian");
        h.insert(vec!["g"], "gay");
        h.insert(vec!["b", "bi"], "bisexual");
        h.insert(vec!["t", "trans"], "transgender");
        h
    };
}

fn get_flag(name: &str) -> Option<Vec<colors::RGBColor>> {
    let mut full_name: &str = name;

    for (al, n) in FLAG_ALIASES.iter() {
        if al.contains(&name) {
            full_name = n;
            break;
        }
    }

    match FLAGS.get(&full_name) {
        Some(v) => Some(v.to_vec()),
        None => None,
    }
}

fn make_esc(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[22;48;2;{};{};{}m", r, g, b)
}

fn make_band(r: u8, g: u8, b: u8) -> String {
    let termsize::Size { rows: _, cols } = termsize::get().unwrap();
    format!("{}{}\x1b[m", make_esc(r, g, b), " ".repeat(cols.into()))
}

fn make_flag(flag: Vec<colors::RGBColor>) -> String {
    let mut flag_string = String::new();
    for (r, g, b) in flag {
        flag_string.extend(format!("{}\n", make_band(r, g, b)).chars());
    }
    flag_string
}

fn main() {
    let args = Args::parse();
    let flag_name = args.get_flag_name();
    let flag = match get_flag(flag_name.as_str()) {
        Some(v) => v,
        None => {
            eprintln!("Error: '{}' is not a valid flag name", flag_name);
            process::exit(1);
        }
    };
    print!("{}", make_flag(flag));
}
