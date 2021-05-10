use itertools::{Itertools, EitherOrBoth};
use sysinfo::{System, SystemExt};
pub use uwuifier;
use structopt::StructOpt;
use unicode_segmentation::UnicodeSegmentation;

use owo_colors::{OwoColorize, Rgb};

mod info;
use info::{InfoBuilder, InfoType};

mod cli;
use cli::Opt;


fn main() {
    let system = System::new_all();
    dbg!(system.get_total_memory());
    let opt: Opt = Opt::from_args();

    let mut info = InfoBuilder::new();
    info.set_color(&opt.color);

    if let Some(values) = opt.values {
        for value in values.iter() {
            info.add(*value);
        }
    } else {
        info.add(InfoType::UserAtHostname)
            .add(InfoType::Os)
            .add(InfoType::Kernel)
            .add(InfoType::Memory)
            .add(InfoType::Processor)
            .add(InfoType::Shell)
            .add(InfoType::Terminal)
            .add(InfoType::RootDisk);
    }

    let (art, size, color) = pad_and_color(match system.get_name().unwrap_or_default().as_str() {
        "Arch Linux" => include_str!("../art/arch"),
        "Manjaro Linux" => include_str!("../art/manjaro"),
        _ => include_str!("../art/default"),
    });

    for elem in art.lines().zip_longest(info.get().iter()) {
        match elem {
            EitherOrBoth::Both(art_line, info_line) => {
                uwu!("{}{}", art_line.color(color), owo!(info_line));
            },
            EitherOrBoth::Left(art_line) => {
                uwu!("{}", art_line.color(color));
            },
            EitherOrBoth::Right(info_line) => {
                for _ in 0..size { print!(" "); }
                uwu!("{}", owo!(info_line));
            }
        }
    }
}

fn pad_and_color(art: &str) -> (String, usize, Rgb) {
    let mut new = String::new();
    let size = art.lines().map(|e| e.graphemes(true).count()).max().unwrap() + 3;
    let hex= hex::decode(art.lines().next().unwrap()).unwrap();
    for line in art.lines().skip(1) {
        let mut line = line.to_string();
        let diff = size - line.graphemes(true).count();
        for _ in 0..diff {
            line.push(' ');
        }
        line.push('\n');
        new.push_str(&line);
    }
    (new, size, Rgb(hex[0], hex[1], hex[2]))
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