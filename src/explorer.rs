use crate::internal::{self, BootResult, BootError};

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

pub struct OwlOptions<'a> {
    pub open: bool,
    pub current: u8,
    pub items: [&'a str; 8]
}

impl<'a> OwlOptions<'a> {
    pub fn new() -> OwlOptions<'a> {
        OwlOptions {
            open: false,
            current: 0,
            items: [
                ":end - quits from the application.",
                ":exp - explore everything inside cwd.",
                ":ser - searches for a given file inside cwd.",
                ":scd - switches the cwd to the given directory.",
                ":del - deletes a given file and moves it to recycle bin.",
                ":cpy - copies a given file.",
                ":opn - opens the contents of a given file.",
                ":mov - moves the given file to a given path."
            ]
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

pub struct Owl<'a> {
    pub state: OwlState,
    pub shell: OwlShell, 
    pub options: OwlOptions<'a>,
    pub cwd: String,
}

impl<'a>  Owl<'a> {
    pub fn new() -> BootResult<Owl<'a>> {
        let cwd: String = match internal::home_drive() {
            Ok(drive) => drive,
            Err(e) => {return Err(BootError::DriveLoadingFailed(e)); }
        };

        let owl = Owl {
            state: OwlState::Normal,
            shell: OwlShell::new(),
            options: OwlOptions::new(),
            cwd: cwd,
        };

        Ok(owl)
    }

    pub fn get_state_desc(&mut self) -> Option<String> {
        match self.state {
            OwlState::Normal | OwlState::OwlShell | OwlState::OwlOptions => {
                let normal_state: String = self.state.to_string();
                Some(format!("{:padding_level$}{normal_state} mode at {}", "", self.cwd, padding_level=1))
            },
            _ => Some(self.state.to_string()),
        }
    }
    
    pub fn append_to_shell(&mut self, pressed: char) {
        self.shell.append(pressed);
        self.shell.cursor_shift_right();
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
        if self.shell.input == ":exp" {
            // TODO: explore all dirs of cwd.
        }
    }
}