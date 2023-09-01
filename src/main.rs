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
        self.flag.unwrap_or(DEFAULT_FLAG.to_owned()).to_lowercase()
    }
}

fn get_flag(name: &str) -> Option<Vec<color::RGBColor>> {
    let mut full_name: &str = name;

    if let Some(&n) = flag::NAME_ALIASES
        .iter()
        .find(|&(al, _)| al.contains(&name))
        .map(|(_, n)| n)
    {
        full_name = n;
    }

    flag::FLAGS.get(&full_name).map(|v| v.to_vec())
}

fn make_esc(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[22;48;2;{};{};{}m", r, g, b)
}

fn make_band(r: u8, g: u8, b: u8) -> String {
    let termsize::Size { rows: _, cols } = termsize::get().expect("could not fetch terminal size");
    format!("{}{}\x1b[m", make_esc(r, g, b), " ".repeat(cols.into()))
}

fn make_flag(flag: Vec<color::RGBColor>) -> String {
    flag.iter()
        .map(|(r, g, b)| make_band(*r, *g, *b))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let args = Args::parse();
    let flag_name = args.get_flag_name();
    let flag = get_flag(flag_name.as_str()).unwrap_or_else(|| {
        eprintln!("Error: '{}' is not a valid flag name", flag_name);
        process::exit(1);
    });

    print!("{}", make_flag(flag));
}
