use itertools::{Itertools, EitherOrBoth};
pub use uwuifier;

mod info;
use info::{InfoBuilder, InfoType};
use sysinfo::{System, SystemExt};

fn main() {
    let system = System::new_all();

    let mut info = InfoBuilder::new();
    info.add(InfoType::UserAtHostname)
        .add(InfoType::Os)
        .add(InfoType::Kernel)
        .add(InfoType::Memory)
        .add(InfoType::Processor)
        .add(InfoType::Shell)
        .add(InfoType::Terminal)
        .add(InfoType::RootDisk);

    let (art, size) = pad(match system.get_name().unwrap_or_default().as_str() {
        "Arch Linux" => include_str!("../art/arch"),
        _ => include_str!("../art/default"),
    });

    for elem in art.lines().zip_longest(info.get().iter()) {
        match elem {
            EitherOrBoth::Both(art_line, info_line) => {
                uwu!("{}{}", art_line, owo!(info_line));
            },
            EitherOrBoth::Left(art_line) => {
                uwu!("{}", art_line);
            },
            EitherOrBoth::Right(info_line) => {
                for _ in 0..size { print!(" "); }
                uwu!("{}", owo!(info_line));
            }
        }
    }
}

fn pad(art: &str) -> (String, usize) {
    let mut new = String::new();
    let size = art.lines().map(|e| e.len()).max().unwrap() + 3;
    for line in art.lines() {
        let mut line = line.to_string();
        let diff = size - line.len();
        for _ in 0..diff {
            line.push_str(" ");
        }
        line.push_str("\n");
        new.push_str(&line);
    }
    (new, size)
}

#[macro_export]
macro_rules! owo {
    ($expression:expr) => {
        $expression.as_ref().unwrap_or(&"?".to_string())
    }
}

#[macro_export]
macro_rules! uwu {
    ($($arg:tt)*) => {{
        println!("{}", uwuifier::uwuify_str_sse(&format!($($arg)*)))
    }}
}