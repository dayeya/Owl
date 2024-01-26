# Owl
Owl is a minimalistic, simple and fast file explorer that aims to make file exploring fast, safe and easy.
To achieve that Owl provides keyboard only TUI, a built-in shell and a built-in guide.

# Normal mode
The default mode of Owl, you can go back to normal mode by pressing the ```Escape``` key.<br />
The normal mode enables you to browse through files and open them by pressing the ```Enter``` key while hovering on the file.

# Shell mode
Owl has its own shell and unique commands to interact with the file system. <br />
You can gain access to the mode by typing ```:``` in any other mode.

## Shell commands
| Command | Description                             | Synopsis                          |
|:--------|:----------------------------------------|:----------------------------------|
| end     | Quit from owl                           | end                               |
| exp     | Display cwd tree                        | exp                               |
| ser     | Searches for a file inside cwd          | ser [file_name.extension]         |
| scd     | Switches the cwd to the given directory | scd [new directory path]          |
| del     | Deletes a file from cwd                 | del [file_name.extension]         |
| cpy     | Copies a file to clipboard              | cpy [file_name.extension]         |
| opn     | Opens the contents of a file            | opn [file_name.extension]         |
| mov     | Moves the file to a given directory     | mov [file_name.extension] \[path] |

# Options mode
Display all available commands in a separate mode. <br />
I've chosen to create a separate mode for this feature as it is more convenient to navigate inside Owl.

# Development Milestones
- [x] Configurable.
- [ ] Preview files.
- [ ] Creation of files and deletion.
- [ ] Very fast resource searching algorithm.
- [ ] Compressing folders and zip extractions.

# Authors
Daniel Sapojnikov 2024.
