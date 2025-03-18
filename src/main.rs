#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod win;
mod wstr;

use std::fs;

use anyhow::Result;
use windows::Win32::System::Threading::PROCESS_POWER_THROTTLING_EXECUTION_SPEED;

fn main() -> Result<()> {
    let conf: Vec<(String, bool)> = fs::read_to_string("conf.txt")?
        .lines()
        .map(String::from)
        .map(|line| {
            if line.contains(',') {
                let pair = line.split_once(',').unwrap();
                (pair.0.to_owned(), pair.1 != "0")
            } else {
                (line, true)
            }
        })
        .collect();
    let processes = win::enum_processes()?;

    for process in &processes {
        if let Some(conf) = conf.iter().find(|v| process.1.to_uppercase().contains(&v.0.to_uppercase())) {
            if conf.1 {
                let _ = win::set_power_throttling_state(process.0, PROCESS_POWER_THROTTLING_EXECUTION_SPEED);
            } else {
                let _ = win::set_power_throttling_state(process.0, 0);
            }
        }
    }

    Ok(())
}
