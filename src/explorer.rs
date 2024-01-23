use std::sync::Arc;
use std::path::PathBuf;
use ratatui::widgets::ListState;
use crate::internal::{self, BootResult, BootError, Directory};

pub enum CursorDirection {Right, Left}

pub enum OwlState {
    Normal,
    Ended,
    OwlShell,
    OwlOptions,
}

impl ToString for OwlState {
    fn to_string(&self) -> String {
        match self {
            OwlState::Normal => String::from("Normal"),
            OwlState::OwlShell => String::from("Shell"),
            OwlState::OwlOptions => String::from("Options"),
            OwlState::Ended => String::from("Ended"),
        }
    }
}

pub struct OwlOptions {
    pub open: bool,
    pub current: u8,
}

impl<'a> OwlOptions {
    pub fn new() -> OwlOptions {
        OwlOptions {
            open: false,
            current: 0,
        }
    }
}

pub struct OwlShell {
    pub input: String,
    pub cursor_position: usize, 
}

impl OwlShell {
    fn new() -> OwlShell {
        OwlShell {
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

pub struct Owl {
    pub state: OwlState,
    pub shell: OwlShell, 
    pub options: OwlOptions,
    pub cwd: Directory
}

impl Owl {
    pub fn new() -> BootResult<Owl> {
        let _drives: Arc<Vec<PathBuf>> = Arc::new(
            match internal::drives() {
                Ok(drives) => drives,
                Err(e) => { return Err(BootError::DriveLoadingFailed(e)); }
            }
        ); 

        let cwd: Arc<PathBuf> = Arc::new(PathBuf::from(
            match internal::home_drive() {
                Ok(drive) => drive,
                Err(e) => { 
                    return Err(BootError::DriveLoadingFailed(e));
                }
            }
        ));

        let owl = Owl {
            state: OwlState::Normal,
            shell: OwlShell::new(),
            options: OwlOptions::new(),
            cwd: Directory::from(cwd),
        };

        Ok(owl)
    }

    pub fn format_mode(&mut self) -> String {
        match self.state {
            OwlState::Normal | OwlState::OwlShell | OwlState::OwlOptions => {
                let normal_state: String = self.state.to_string();
                format!("{:padding_level$}{normal_state} mode at {}", "", self.cwd, padding_level=1)
            },
            _ => self.state.to_string(),
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

    pub fn execute_shell(&mut self) {
        if self.shell.input == ":end" {
            self.state = OwlState::Ended;
        }
        else if self.shell.input == ":exp" {
            // TODO: explore all dirs of cwd.
        }
        else {
            self.shell.input = String::from("Unknown Command");
        }   
    }

    // Displays all contents of cwd.
    pub fn walk(&mut self) -> Vec<String> {
        self.cwd.walk()
    }
}