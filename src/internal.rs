// Amazing source: https://profpatsch.de/notes/rust-string-conversions.

use std::fs;
use std::io;
use std::fmt;
use std::ffi::OsStr;
use std::fs::Permissions;
use std::error::Error;
use std::sync::Arc;
use std::time::SystemTime;
use std::path::{PathBuf, Display};
use chrono::offset::Utc;
use chrono::DateTime;
use humansize::{make_format, DECIMAL};
use winsafe::{self as w, co::ERROR, SysResult};

#[derive(Debug, Clone)]
pub enum BootError {
    DriveLoadingFailed(ERROR),
    ConfigLoadingFailed
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BootError::DriveLoadingFailed(e) => write!(f, "Drive loading failed, {e}"),
            BootError::ConfigLoadingFailed => write!(f, "Config loading failed.")
        }
    }
}

impl Error for BootError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BootError::DriveLoadingFailed(ref e) => Some(e),
            BootError::ConfigLoadingFailed => None
        }
    }
}

pub type BootResult<T> = Result<T, BootError>;

pub(crate) fn drives() -> SysResult<Vec<PathBuf>> {
    if w::GetLogicalDrives() == 0 {
        return Err(w::GetLastError())
    }
    let logical_drives: Vec<String> = w::GetLogicalDriveStrings()?;
    Ok(logical_drives.into_iter().map(|p| PathBuf::from(p)).collect::<Vec<PathBuf>>())
}

pub(crate) fn home_drive() -> SysResult<PathBuf> {
    let available_drives: Vec<PathBuf> = drives()?;
    let home_drive: &PathBuf = available_drives.first().unwrap();
    Ok(home_drive.to_owned())
}

pub(crate) fn human_time(sys_time: io::Result<SystemTime>) -> io::Result<String> {
    let datetime: DateTime<Utc> = DateTime::from(sys_time?);
    let formatted: String = format!("{}", datetime.format("%d/%m/%Y %H:%M"));
    Ok(formatted)
}

pub(crate) fn human_size(size: u64) -> String {
    let formatter = make_format(DECIMAL);
    formatter(size)
}

pub type DateModified = io::Result<String>;
pub type DateAccessed = io::Result<String>;
pub type DateCreation = io::Result<String>;

pub struct Node {
    pub root_path: Arc<PathBuf>,
    pub size: u64,
    pub is_file: bool,
    pub is_dir: bool,
    pub extension: String,
    pub created: DateCreation,
    pub accessed: DateAccessed,
    pub modified: DateModified,
    pub permissions: Option<Permissions>
}

impl Node {
    pub fn from(root_path: Arc<PathBuf>) -> Self {
        let (
            size, 
            is_file, 
            is_dir, 
            extension,
            created, 
            accessed, 
            modified, 
            permissions
        ) = root_path.metadata()
        .map(
            |md| {
                (
                    md.len(),
                    md.is_file(),
                    md.is_dir(),
                    root_path.extension().unwrap_or(OsStr::new("Folder")).to_string_lossy().to_string(),
                    human_time(md.created()),
                    human_time(md.accessed()),
                    human_time(md.modified()),
                    Some(md.permissions())
                )
            }
        ).unwrap_or((
            0, 
            false, 
            false, 
            "".to_string(), 
            Ok("Unresolvable".to_string()),
            Ok("Unresolvable".to_string()),
            Ok("Unresolvable".to_string()),
            None
        ));

        Self {
            root_path,
            size,
            is_file,
            is_dir,
            extension,
            created,
            accessed,
            modified,
            permissions
        }
    }

    pub fn name(&self) -> &OsStr {
        // Doesnt matter if 'Self' is a dir of a file.
        self.root_path.file_name().unwrap_or(OsStr::new("").as_ref())
    }
}

pub struct Directory {
    parent: Arc<PathBuf>,
    nodes: Vec<Node>
}

impl Directory {
    pub fn from(path: Arc<PathBuf>) -> Self {
        let parent = path;
        let nodes: Vec<Node> = fs::read_dir(parent.as_ref()).unwrap().map(
            |rd| rd.map(|e| Node::from(Arc::new(e.path()))).unwrap()
        ).collect();
        
        Self {
            parent,
            nodes,
        }
    }

    pub fn walk(&mut self) -> Vec<[String; 4]> {
        self.nodes.iter().map(
            |n| [
                n.name().to_string_lossy().to_string(), 
                n.modified.as_ref().unwrap().to_owned(), 
                n.extension.as_str().to_owned(), 
                human_size(n.size)
            ]
        ).collect::<Vec<[String; 4]>>()
    }

    pub fn display(&self) -> Display<'_> {
        self.parent.display()
    }
}


/*
// File Access Rights - docs: https://learn.microsoft.com/en-us/windows/win32/fileio/file-access-rights-constants.
pub enum FCR {
    FileAddFile = 0x4,
    FileAddSubdirectory = 0x4,
    FileAllAccess, 
    FileAppendData = 0x4,
    FileCreatePipeInstance,
    FileDeleteChild = 0x40,
    FileExecute = 0x20,
    FileListDirectory = 0x1,
    FileReadAttributes = 0x80,
    FileReadData = 0x1,
    FileReadEa = 0x8,
    FileTraverse = 0x20,
    FileWriteAttributes = 0x100,
    FileWriteData = 0x2,
    FileWriteEa = 0x10,
    StandardRightsRead,
    StandardRightsWrite
}
*/