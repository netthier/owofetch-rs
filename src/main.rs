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
use textwrap::{termwidth, wrap};
use uwuifier::uwuify_str_sse;


fn main() {
    let system = System::new_all();
    let opt: Opt = Opt::from_args();

    let mut info = InfoBuilder::new(&opt);
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
        "Darwin" => include_str!("../art/apple"),
        _ => include_str!("../art/default"),
    });

    let info = info.get();

    for elem in art.lines().zip_longest(format_info(&info, size)) {
        match elem {
            EitherOrBoth::Both(art_line, info_line) => {
                uwu!("   {}", art_line.color(color));
                println!("{}", info_line);
            },
            EitherOrBoth::Left(art_line) => {
                uwu!("   {}", art_line.color(color));
            },
            EitherOrBoth::Right(info_line) => {
                for _ in 0..size+3 { print!(" "); }
                println!("{}", info_line);
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

fn format_info(info: &[Option<String>], art_size: usize) -> Vec<String>{
    let width = termwidth() - (art_size + 3);
    let mut new_info = Vec::new();
    for line in info.iter() {
        if let Some(info) = line {
            new_info.append(&mut wrap(&uwuify_str_sse(&info), width).iter().map(|e| String::from(&*e.clone())).collect());
        }
    }
    new_info
}

#[macro_export]
macro_rules! uwu {
    ($($arg:tt)*) => {{
        print!("{}", uwuifier::uwuify_str_sse(&format!($($arg)*)))
    }}
}
