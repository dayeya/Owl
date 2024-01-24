use ratatui::prelude::Color;

pub const OWL_BACKGROUND: Color = Color::Rgb(23, 23, 23);
pub const OWL_FONT_COLOR: Color = Color::Rgb(254, 250, 212);

pub const OPTIONS: [&str; 8] = [
    ":end - quits from the application.",
    ":exp - explore everything inside cwd.",
    ":ser - searches for a given file inside cwd.",
    ":scd - switches the cwd to the given directory.",
    ":del - deletes a given file and moves it to recycle bin.",
    ":cpy - copies a given file.",
    ":opn - opens the contents of a given file.",
    ":mov - moves the given file to a given path."
];