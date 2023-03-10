pub mod colors;
pub mod flags;

use crate::{colors::color, flags::flag};

use std::process;

use clap::Parser;

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

fn get_flag(name: &str) -> Option<Vec<color::RGBColor>> {
    let mut full_name: &str = name;

    for (al, n) in flag::NAME_ALIASES.iter() {
        if al.contains(&name) {
            full_name = n;
            break;
        }
    }

    match flag::FLAGS.get(&full_name) {
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

fn make_flag(flag: Vec<color::RGBColor>) -> String {
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
