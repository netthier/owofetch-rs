use std::env;
use sysinfo::{System, SystemExt, ProcessExt, ProcessorExt, DiskExt};
use std::process;
use std::ffi::OsString;
use std::str::FromStr;
use owo_colors::{OwoColorize, Rgb};
use strum::EnumString;
use english_numbers::{Formatting, convert};
use crate::cli::Opt;

#[derive(Debug, EnumString, Copy, Clone)]
pub enum InfoType {
    UserAtHostname,
    Os,
    Kernel,
    Memory,
    Shell,
    Terminal,
    Processor,
    RootDisk,
}

pub struct InfoBuilder {
    info: Vec<InfoType>,
    system: System,
    num_fmt_params: Formatting,
    word_color: Rgb,
    config: Opt,
}

impl InfoBuilder {
    pub fn new(opt: &Opt) -> Self {
        Self {
            info: Vec::new(),
            system: System::new_all(),
            num_fmt_params: Formatting {
                spaces: true,
                conjunctions: true,
                ..Default::default()
            },
            word_color: Rgb(0xFF, 0xFF, 0xFF),
            config: opt.clone(),
        }
    }

    pub fn add(&mut self, info_type: InfoType) -> &mut Self {
        self.info.push(info_type);
        self
    }

    pub fn set_color(&mut self, hex: &str) -> &mut Self {
        let hex= hex::decode(hex.chars().filter(|c| c.is_alphanumeric()).collect::<String>()).expect("Invalid hex color!");
        self.word_color = Rgb(hex[0], hex[1], hex[2]);
        self
    }

    pub fn get(&self) -> Vec<Option<String>> {
        let mut info_vec = Vec::new();
        for info in self.info.iter() {
            info_vec.push(match info {
                InfoType::UserAtHostname => self.get_user_at_host(),
                InfoType::Os => self.get_os(),
                InfoType::Kernel => self.get_kernel(),
                InfoType::Memory => self.get_memory(),
                InfoType::Shell => self.get_shell(),
                InfoType::Terminal => self.get_term(),
                InfoType::Processor => self.get_cpu(),
                InfoType::RootDisk => self.get_root_disk(),
            });
        }
        info_vec
    }

    fn num_fmt(&self, mut num: f32) -> String {
        if self.config.nums {
            format!("{:.2}", num)
        } else {
            let mut decimals = 0;
            while num > 99.0 {
                num /= 10.0;
                decimals += 1;
            }
            format!("About {}", convert(num as i64*10_i64.pow(decimals), self.num_fmt_params))
        }
    }

    fn get_os(&self) -> Option<String> {
        Some(format!("{} {}", "OS:".color(self.word_color), self.system.get_name()?))
    }

    fn get_kernel(&self) -> Option<String> {
        Some(format!("{} {}", "Kernel".color(self.word_color), self.system.get_kernel_version()?))
    }

    fn get_memory(&self) -> Option<String> {
        let total = self.system.get_total_memory() as f32 / 1000.0;
        let used = self.system.get_used_memory() as f32 / 1000.0;
        Some(format!("{} {} Megabytes / {} Megabytes", "Memory:".color(self.word_color), self.num_fmt(used), self.num_fmt(total)))
    }

    fn get_user_at_host(&self) -> Option<String> {
        Some(format!("{}@{}", env::var("USER").ok()?.color(self.word_color), self.system.get_host_name()?.color(self.word_color)))
    }

    fn get_shell(&self) -> Option<String> {
        Some(format!("{} {}", "Shell:".color(self.word_color), env::var("SHELL").ok()?))
    }

    fn get_term(&self) -> Option<String> {
        let pid = process::id();
        let process = self.system.get_process(pid as i32)?;
        let shell = self.system.get_process(process.parent()?)?;
        let terminal = self.system.get_process(shell.parent()?)?;
        Some(format!("{} {}", "Terminal: ".color(self.word_color), terminal.name()))
    }

    fn get_cpu(&self) -> Option<String> {
        let processor = self.system.get_processors().iter().next()?;
        Some(format!("{} {}", "CPU: ".color(self.word_color), processor.get_brand()))
    }

    fn get_root_disk(&self) -> Option<String> {
        let disk = self.system.get_disks().iter().find(|e| e.get_mount_point() == OsString::from_str("/").unwrap())?;
        let total = disk.get_total_space() as f32 / 1073741824.0;
        let used = total - disk.get_available_space() as f32 / 1073741824.0;

        Some(format!("{} {} Gibibytes / {} Gibibytes", "Disk:".color(self.word_color), self.num_fmt(used) ,self.num_fmt( total) ))
    }
}