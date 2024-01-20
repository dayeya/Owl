use winsafe::{
    self as w, co::{DRIVE, ERROR},
    SysResult
};
use std::fmt;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum BootError {
    DriveLoadingFailed(ERROR)
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BootError::DriveLoadingFailed(e) => write!(f, "Drive loading failed due: {e}\nplease reload Owl."),
        }
    }
}

impl Error for BootError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            BootError::DriveLoadingFailed(ref e) => Some(e),
        }
    }
}

pub type BootResult<T> = Result<T, BootError>;

pub struct DriveInfo {
    root_path: Box<Path>,
    drive_type: DRIVE,
    space_info: String
}

pub(crate) fn drives() -> SysResult<Vec<String>> {
    if w::GetLogicalDrives() == 0 {
        return Err(w::GetLastError())
    }
    w::GetLogicalDriveStrings()
}

pub(crate) fn home_drive() -> SysResult<String> {
    // The current working directory of owl depends on the first driver.
    // Owl defines the first fetched driver as the `DEFUALT` one.
    let available_drives: Vec<String> = drives()?;
    let home_drive: &String = available_drives.first().unwrap();
    Ok(home_drive.to_owned())
}

/*
fn disk_space_info(root_path: Option<&str>) -> DiskSpaceInfo {
    let mut free_bytes_available_to_calles: Option<&mut u64>;
    let mut total_number_of_bytes: Option<&mut u64>;
    let mut total_number_of_free_bytes :Option<&mut u64>;

    w::GetDiskFreeSpaceEx(
        root_path, 
        free_bytes_available_to_calles,
        total_number_of_bytes,
        total_number_of_free_bytes
    );
}

pub(crate) fn get_driver_info() -> SysResult<Vec<DRIVE>> {
    let available_drives: Vec<String> = drives()?;
    available_drives.into_iter()
    .map(|drive_path: String| {
        let root_path: Box<String> = Box::new(drive_path);
        let drive_type = w::GetDriveType(Some(&root_path));
        DriveInfo::from(
            root_path,
            drive_type,
            w::GetDiskFreeSpaceEx(&root_path)
        )
    });
} */