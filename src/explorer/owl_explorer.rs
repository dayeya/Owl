pub struct Options<'a> {
    pub items: Vec<Vec<&'a str>>
}

impl<'a> Options<'a> {
    pub fn new() -> Options<'a> {
        Options {
            items: vec![
                vec![":", "opens shell prompt"],
                vec![":exp", "starts exploring"],
                vec![":end", "quits from owl"],
                vec![":del", "delets a chosen file from cwd"],
                vec![":cpy", "copies a chosen file to clipboard"],
                vec![":opn", "opens the contents of a chosen file"]
            ]
        }
    }
}

pub struct Owl<'a> {
    pub inside_options: bool, 
    pub inside_shell: bool,
    pub options: Options<'a>,
    // current_working_dir: String,
}

impl<'a>  Owl<'a> {
    pub fn new() -> Owl<'a> {
        Owl {
            inside_options: false, 
            inside_shell: false,
            options: Options::new(),
        }
    }
}