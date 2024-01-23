// Amazing source: https://profpatsch.de/notes/rust-string-conversions.

use std::fs;
use std::io;
use std::fmt;
use std::fs::Permissions;
use std::sync::Arc;
use std::time::SystemTime;
use std::path::{PathBuf, Display};
use std::error::Error;
use chrono::offset::Utc;
use chrono::DateTime;
use winsafe::{self as w, co::ERROR, SysResult};

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

pub(crate) fn drives() -> SysResult<Vec<PathBuf>> {
    if w::GetLogicalDrives() == 0 {
        return Err(w::GetLastError())
    }
    let logical_drives: Vec<String> = w::GetLogicalDriveStrings()?;
    Ok(logical_drives.into_iter().map(|p| PathBuf::from(p)).collect::<Vec<PathBuf>>())
}

pub(crate) fn home_drive() -> SysResult<PathBuf> {
    // The current working directory of owl depends on the first driver.
    // Owl defines the first fetched driver as the `DEFUALT` one.
    let available_drives: Vec<PathBuf> = drives()?;
    let home_drive: &PathBuf = available_drives.first().unwrap();
    Ok(home_drive.to_owned())
}

pub(crate) fn human_time(sys_time: io::Result<SystemTime>) -> io::Result<String> {
    let datetime: DateTime<Utc> = DateTime::from(sys_time?);
    let formatted: String = format!("{}", datetime.format("%d/%m/%Y %H:%M"));
    Ok(formatted)
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
    pub fn from(path: Arc<PathBuf>) -> Self {
        let (
            size, 
            is_file, 
            is_dir, 
            extension,
            created, 
            accessed, 
            modified, 
            permissions
        ) = path.metadata()
        .map(
            |md| {
                (
                    md.len(),
                    md.is_file(),
                    md.is_dir(),
                    path.extension().unwrap_or_default().to_string_lossy().to_string(),
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
            Ok("Unresolveable".to_string()), 
            Ok("Unresolveable".to_string()), 
            Ok("Unresolveable".to_string()), 
            None
        ));

        Self {
            root_path: path,
            size: size,
            is_file: is_file,
            is_dir: is_dir,
            extension: extension,
            created: created,
            accessed: accessed,
            modified: modified,
            permissions: permissions
        }
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
            parent: parent,
            nodes: nodes,
        }
    }

    pub fn walk(&mut self) -> Vec<String> {
        let walked_node: Vec<String> = self.nodes.iter().map(
            |n| n.root_path.to_string_lossy().to_string()
        ).collect::<Vec<String>>();

        walked_node
    }

    pub fn display(&self) -> Display<'_> {
        self.parent.display()
    }
}

// TODO: Get info of node.
// TODO: Get info of drives.


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