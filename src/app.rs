use std::fmt;
use std::sync::Arc;
use std::path::PathBuf;
use crate::internal::{self, BootResult, BootError, Directory};
use crate::config::Config;

pub enum CursorDirection {Right, Left}

pub enum Mode {
    Normal,
    Ended,
    InsideShell,
    InsideOptions,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Mode::Normal => "NOR".to_string(),
            Mode::InsideShell => "SHL".to_string().to_string(),
            Mode::InsideOptions => "OPS".to_string(),
            Mode::Ended => "END".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub struct AppOptions {
    pub open: bool,
    pub current: u8,
}

impl<'a> AppOptions {
    pub fn new() -> AppOptions {
        AppOptions {
            open: false,
            current: 0,
        }
    }
}

pub struct AppShell {
    pub input: String,
    pub cursor_position: usize,
}

impl AppShell {
    fn new() -> AppShell {
        AppShell {
            input: ":".to_string(),
            cursor_position: 1,
        }
    }

    fn append(&mut self, pressed: char) {
        self.input.insert(self.cursor_position, pressed)
    }

    fn delete(&mut self) {
        if self.cursor_position > 1 {
            match self.input.pop() {
                Some(_) => self.cursor_shift_left(),
                _ => {}, 
            }
        }
    }

    fn cursor_shift_left(&mut self) {
        let new_cursor_pos: usize = self.cursor_position.saturating_sub(1);
        self.cursor_position = new_cursor_pos.clamp(0, self.input.len());
    }

    fn cursor_shift_right(&mut self) {
        let new_cursor_pos: usize = self.cursor_position.saturating_add(1);
        self.cursor_position = new_cursor_pos.clamp(0, self.input.len());
    }
}

pub struct App {
    pub config: Config,
    pub mode: Mode,
    pub shell: AppShell, 
    pub options: AppOptions,
    pub cwd: Directory,
    pub selection_idx: Option<usize>
}

impl App {
    pub fn new() -> BootResult<Self> {
        let cnf = Config::new().unwrap();

        let _drives: Arc<Vec<PathBuf>> = Arc::new(
            match internal::drives() {
                Ok(drives) => drives,
                Err(e) => { return Err(BootError::DriveLoadingFailed(e)); }
            }
        ); 

        let cwd: Arc<PathBuf> = Arc::new(PathBuf::from(
            match internal::home_drive() {
                Ok(drive) => drive,
                Err(e) => { return Err(BootError::DriveLoadingFailed(e)); }
            }
        ));

        let app = Self {
            config: cnf,
            mode: Mode::Normal,
            shell: AppShell::new(),
            options: AppOptions::new(),
            cwd: Directory::from(cwd),
            selection_idx: Some(0),
        };

        Ok(app)
    }

    pub fn format_mode(&mut self) -> String {
        let app_mode: String = self.mode.to_string();
        let formatted: String = format!("{:spacing_before$}{app_mode}{:spacing_between$}{}",
                                        "", "", self.cwd.display(), spacing_before=1, spacing_between=3);
        formatted
    }
    
    pub fn execute_shell(&mut self) {
        if self.shell.input == ":end" {
            self.mode = Mode::Ended;
        }
        else if self.shell.input == ":exp" {
            // TODO: explore all dirs of cwd.
        }
        else {
            self.shell.input = String::from("Unknown Command");
        }   
    }
    
    pub fn append_to_shell(&mut self, pressed: char) {
        if pressed == ':' {
            self.reset_shell();
        } else { 
            self.shell.append(pressed);
            self.shell.cursor_shift_right();
        }
    }

    pub fn delete_from_shell(&mut self) {
        self.shell.delete();
    }

    pub fn reset_shell(&mut self) {
        self.shell.input = ":".to_string();
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        match direction {
            CursorDirection::Left => self.shell.cursor_shift_left(),
            CursorDirection::Right => self.shell.cursor_shift_right(),
        }
    }

    pub fn shift_down(&mut self) {
        let i = match self.selection_idx {
            Some(k) => {
                let dir = self.cwd.walk();
                if k < dir.len() {
                    k + 1
                } else {
                    0
                }
            }
            None => 0
        };
        self.selection_idx = Some(i);
    }

    pub fn shift_up(&mut self) {
        let i = match self.selection_idx {
            Some(k) => {
                if k > 0 {
                    k - 1
                } else {
                    let dir = self.cwd.walk();
                    dir.len() - 1
                }
            }
            None => 0
        };
        self.selection_idx =  Some(i)
    }
}