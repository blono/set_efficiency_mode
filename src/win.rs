use std::mem;

use anyhow::{bail, Result};
use scopeguard::defer;
use windows::{core::PWSTR, Win32::{Foundation::CloseHandle, System::{ProcessStatus::EnumProcesses, Threading::{OpenProcess, ProcessPowerThrottling, QueryFullProcessImageNameW, SetPriorityClass, SetProcessInformation, IDLE_PRIORITY_CLASS, PROCESS_NAME_WIN32, PROCESS_POWER_THROTTLING_CURRENT_VERSION, PROCESS_POWER_THROTTLING_EXECUTION_SPEED, PROCESS_POWER_THROTTLING_STATE, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SET_INFORMATION}}}};

use crate::wstr;

pub fn query_full_process_path(process_id: u32) -> Result<String> {
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id)?;
        defer! {
            let _ = CloseHandle(process);
        }

        let mut len = 4096u32;
        let mut path = [0u16; 4096];

        QueryFullProcessImageNameW(process, PROCESS_NAME_WIN32, PWSTR::from_raw(path.as_mut_ptr()), &mut len)?;

        return Ok(wstr::decode_utf16(&path[0..len as usize]));
    }
}

pub fn enum_processes() -> Result<Vec<(u32, String)>> {
    let mut processes = vec![0; 4096];
    let mut num_processes: u32 = 0;

    if let Err(e) = unsafe { EnumProcesses(processes.as_mut_ptr(), (mem::size_of::<u32>() * processes.len()) as u32, &mut num_processes) } {
        bail!(e);
    }

    num_processes /= mem::size_of::<u32>() as u32;

    let mut result = Vec::with_capacity(num_processes as usize);

    for i in 0..num_processes {
        let pid = processes[i as usize];

        if let Ok(path) = query_full_process_path(pid) {
            result.push((pid, path));
        }
    }

    Ok(result)
}

/// `state` - Efficiency mode
/// To turn off Efficiency mode, pass 0 for `state`
/// To turn on Efficiency mode, pass `PROCESS_POWER_THROTTLING_EXECUTION_SPEED` for `state`
pub fn set_power_throttling_state(process_id: u32, state: u32) -> Result<()> {
    unsafe {
        let process = OpenProcess(PROCESS_SET_INFORMATION, false, process_id)?;
        defer! {
            let _ = CloseHandle(process);
        }

        if state == PROCESS_POWER_THROTTLING_EXECUTION_SPEED {
            SetPriorityClass(process, IDLE_PRIORITY_CLASS)?;
        }

        let state = PROCESS_POWER_THROTTLING_STATE {
            Version: PROCESS_POWER_THROTTLING_CURRENT_VERSION,
            ControlMask: state,
            StateMask: state,
        };

        Ok(SetProcessInformation(process, ProcessPowerThrottling, &state as *const _ as *const _, mem::size_of::<PROCESS_POWER_THROTTLING_STATE>() as u32)?)
    }
}
