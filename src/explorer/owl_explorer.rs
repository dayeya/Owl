pub enum OwlOption {
    Null, 
    Explore(String),
    Quit(String),
    DeleteFile(String),
    CopyFile(String),
    ShowFile(String),
    SearchFile(String),
}

pub struct OwlOptions<'a> {
    pub current: OwlOption,
    pub items: Vec<Vec<&'a str>>
}

impl<'a> OwlOptions<'a> {
    pub fn new() -> OwlOptions<'a> {
        OwlOptions {
            current: OwlOption::Null,
            items: vec![
                vec![":", "opens shell prompt"],
                vec![":exp", "starts exploring"],
                vec![":end", "quits from owl"],
                vec![":del", "delets a chosen file from cwd"],
                vec![":cpy", "copies a chosen file to clipboard"],
                vec![":opn", "opens the contents of a chosen file"],
                vec![":ser", "searchs for a given file :ser <file>"]
            ]
        }
    }
}

pub enum OwlState {
    Normal,
    Ended,
    OwlShell,
    OwlOptions,
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

        if self.cursor_position <= 1 { return }

        match self.input.pop() {
            Some(_) => self.cursor_shift_left(),
            _ => {}, 
        }
    }

    fn cursor_shift_left(&mut self) {
        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    fn cursor_shift_right(&mut self) {
        self.cursor_position = self.cursor_position.saturating_add(1);
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
    
    pub fn append_to_shell(&mut self, pressed: char) {
        self.shell.append(pressed);
        self.shell.cursor_shift_right();
    }

    pub fn delete_from_shell(&mut self) {
        self.shell.delete();
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        match direction {
            CursorDirection::Left => self.shell.cursor_shift_left(),
            CursorDirection::Right => self.shell.cursor_shift_right(),
        }
    }
}