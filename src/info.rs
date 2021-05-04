use std::env;
use sysinfo::{System, SystemExt, ProcessExt, ProcessorExt, DiskExt};
use std::process;
use std::ffi::OsString;
use std::str::FromStr;

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
}

impl InfoBuilder {
    pub fn new() -> Self {
        Self {
            info: Vec::new(),
            system: System::new_all(),
        }
    }

    pub fn add(&mut self, info_type: InfoType) -> &mut Self {
        self.info.push(info_type);
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
        Some(format!("OS: {}", self.system.get_name()?))
    }

    fn get_kernel(&self) -> Option<String> {
        Some(format!("Kernel: {}", self.system.get_kernel_version()?))
    }
    fn get_memory(&self) -> Option<String> {
        let total = self.system.get_total_memory();
        let used = self.system.get_used_memory();
        Some(format!("Memory: {:.2}MiB / {:.2}MiB", used as f32 / 1024.0, total as f32 / 1024.0))
    }

    fn get_user_at_host(&self) -> Option<String> {
        Some(format!("{}@{}", env::var("USER").ok()?, self.system.get_host_name()?))
    }

    fn get_shell(&self) -> Option<String> {
        Some(format!("Shell: {}", env::var("SHELL").ok()?))
    }

    fn get_term(&self) -> Option<String> {
        let pid = process::id();
        let process = self.system.get_process(pid as i32)?;
        let shell = self.system.get_process(process.parent()?)?;
        let terminal = self.system.get_process(shell.parent()?)?;
        Some(format!("Terminal: {}", terminal.name()))
    }

    fn get_cpu(&self) -> Option<String> {
        let processor = self.system.get_processors().iter().next()?;
        Some(format!("CPU: {}", processor.get_brand()))
    }

    fn get_root_disk(&self) -> Option<String> {
        let disk = self.system.get_disks().iter().filter(|e| e.get_mount_point() == OsString::from_str("/").unwrap()).next()?;
        let total = disk.get_total_space();
        let used = total - disk.get_available_space();

        Some(format!("Disk: {:.2}GiB / {:.2}GiB", used as f32 / 1073741824.0, total as f32 / 1073741824.0))
    }
}