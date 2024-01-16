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
    pub items: [&'a str; 6]
}

impl<'a> OwlOptions<'a> {
    pub fn new() -> OwlOptions<'a> {
        OwlOptions {
            open: false,
            current: 0,
            items: [
                ":exp starts exploring",
                ":end quits from owl",
                ":del delets a chosen file from cwd",
                ":cpy copies a chosen file to clipboard",
                ":opn opens the contents of a chosen file",
                ":ser searchs for a given file :ser <file>"
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

pub enum CursorDirection {
    Right, 
    Left
}

pub struct Owl<'a> {
    pub state: OwlState,
    pub shell: OwlShell, 
    pub options: OwlOptions<'a>,
    // current_working_dir: String,
}

impl<'a>  Owl<'a> {
    pub fn new() -> Owl<'a> {
        Owl {
            state: OwlState::Normal,
            shell: OwlShell::new(),
            options: OwlOptions::new(),
        }
    }

    pub fn set_options(&mut self) {
        self.options.open = true;
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
}