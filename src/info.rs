use std::env;
use sysinfo::{System, SystemExt, ProcessExt, ProcessorExt, DiskExt};
use std::process;
use std::ffi::OsString;
use std::str::FromStr;
use owo_colors::{OwoColorize, Rgb};
use strum::EnumString;
use std::fs::read;

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
    word_color: Rgb,
}

impl InfoBuilder {
    pub fn new() -> Self {
        Self {
            info: Vec::new(),
            system: System::new_all(),
            word_color: Rgb(0xFF, 0xFF, 0xFF),
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

    fn get_os(&self) -> Option<String> {
        Some(format!("{} {}", "OS:".color(self.word_color), self.system.get_name()?))
    }

    fn get_kernel(&self) -> Option<String> {
        Some(format!("{} {}", "Kernel".color(self.word_color), self.system.get_kernel_version()?))
    }
    fn get_memory(&self) -> Option<String> {
        // sysinfo has rounding errors, so we're rolling our own ram method, using sysinfo as a fallback
        let mut total = self.system.get_total_memory();
        let mut available = self.system.get_available_memory();
        if let Ok(data) = read("/proc/meminfo") {
            let string = String::from_utf8_lossy(&data);
            for line in string.split('\n') {
                let field = match line.split(':').next() {
                    Some("MemTotal") => &mut total,
                    Some("MemAvailable") => &mut available,
                    _ => continue,
                };
                if let Some(val_str) = line.rsplit(' ').nth(1) {
                    if let Ok(value) = u64::from_str(val_str) {
                        *field = value
                    }
                }
            }
        }
        Some(format!("{} {:.2}MiB / {:.2}MiB", "Memory:".color(self.word_color), (total - available) as f32 / 1024.0, total as f32 / 1024.0))
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
        Some(format!("{} {}", "Terminal".color(self.word_color), terminal.name()))
    }

    fn get_cpu(&self) -> Option<String> {
        let processor = self.system.get_processors().iter().next()?;
        Some(format!("{} {}", "CPU: ".color(self.word_color), processor.get_brand()))
    }

    fn get_root_disk(&self) -> Option<String> {
        let disk = self.system.get_disks().iter().filter(|e| e.get_mount_point() == OsString::from_str("/").unwrap()).next()?;
        let total = disk.get_total_space();
        let used = total - disk.get_available_space();

        Some(format!("{} {:.2}GiB / {:.2}GiB", "Disk:".color(self.word_color), used as f32 / 1073741824.0, total as f32 / 1073741824.0))
    }
}