#[cfg(windows)]
use log::info;
#[cfg(windows)]
use std::io;
#[cfg(windows)]
use std::mem;
#[cfg(windows)]
use windows::Win32::Foundation::{CloseHandle, HANDLE};
#[cfg(windows)]
use windows::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
    SetInformationJobObject,
};
#[cfg(windows)]
use windows::Win32::System::Threading::OpenProcess;
#[cfg(windows)]
use windows::Win32::System::Threading::PROCESS_SET_QUOTA;
#[cfg(windows)]
use windows::Win32::System::Threading::PROCESS_TERMINATE;

/// A Windows Job Object that kills all associated processes when dropped.
#[cfg(windows)]
pub struct JobObject {
    handle: HANDLE,
}

#[cfg(windows)]
impl JobObject {
    /// Creates a new job object and configures it to kill all processes when the job is closed.
    pub fn new() -> io::Result<Self> {
        unsafe {
            // Create an anonymous job object
            let handle = CreateJobObjectW(None, None).map_err(|e| {
                io::Error::other(
                    format!("CreateJobObjectW failed: {}", e),
                )
            })?;

            // Configure the job to kill all processes when the job handle is closed
            let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = mem::zeroed();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

            SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &info as *const _ as *const _,
                mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
            .map_err(|e| {
                CloseHandle(handle).ok();
                io::Error::other(
                    format!("SetInformationJobObject failed: {}", e),
                )
            })?;

            info!("Created job object with auto-kill on close");
            Ok(JobObject { handle })
        }
    }

    /// Assigns a process to this job object.
    pub fn assign_process(&self, pid: u32) -> io::Result<()> {
        unsafe {
            let process_handle = OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, false, pid)
                .map_err(|e| {
                    io::Error::other(format!("OpenProcess failed: {}", e))
                })?;

            let result = AssignProcessToJobObject(self.handle, process_handle).map_err(|e| {
                CloseHandle(process_handle).ok();
                io::Error::other(
                    format!("AssignProcessToJobObject failed: {}", e),
                )
            });

            CloseHandle(process_handle).ok();
            result?;

            info!("Assigned process {} to job object", pid);
            Ok(())
        }
    }
}

#[cfg(windows)]
impl Drop for JobObject {
    fn drop(&mut self) {
        unsafe {
            info!("Closing job object - all child processes will be terminated");
            CloseHandle(self.handle).ok();
        }
    }
}

// Non-Windows stub
#[cfg(not(windows))]
pub struct JobObject;

#[cfg(not(windows))]
impl JobObject {
    pub fn new() -> std::io::Result<Self> {
        Ok(JobObject)
    }

    pub fn assign_process(&self, _pid: u32) -> std::io::Result<()> {
        Ok(())
    }
}
