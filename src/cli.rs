use structopt::StructOpt;
use crate::info::InfoType;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "owofetch", about = "uwuified *fetch alternative")]
pub struct Opt {
    /// Sets the color of info categories, doesnt affect distro logo or the actual info itself. Must be in RGB hexadecimal format.
    #[structopt(short, long, default_value = "#FFA500")]
    pub color: String,

    /// Only show certain info. Available options are: UserAtHostname, Os, Kernel, Memory, Shell, Terminal, Processor and RootDisk.
    #[structopt(short, long)]
    pub values: Option<Vec<InfoType>>,

    /// Print numeric values as actual, accurate numbers instead of English words.
    #[structopt(short, long)]
    pub nums: bool,
}