# Owl
Owl is a minimalistic, simple and fast file explorer that aims to make file exploring fast, safe and easy.
To achieve that Owl provides keyboard only TUI, a built-in shell and a built-in guide.

# Normal mode
The default mode of Owl, you can go back to normal mode by pressing the ```Escape``` key.
The normal mode enables you to browse through files and open them by pressing the ```Enter``` key while hovering on the file.

# Shell mode
Owl has its own shell and unique commands to interact with the file system. <br />
You can gain access to the mode by typing ```:``` in normal mode.

## Shell commands
- :end - quits from the application.
- :exp - explore everything inside cwd.
- :ser - searches for a given file inside cwd.
- :scd - switches the cwd to the given directory.
- :del - deletes a given file and moves it to recycle bin.
- :cpy - copies a given file.
- :opn - opens the contents of a given file.
- :mov - moves the given file to a given path.

## Sypnosis
- :end
- :exp
- :ser [file_name.extension]
- :scd [new directory path]
- :del [file_name.extension]
- :cpy [file_name.extension]
- :opn [file_name.extension]
- :mov [file.extension] [to]

# Options mode
TODO

# Future-Ideas
* Implement customization.

# Authors
Daniel Sapojnikov 2024.
