# BAUP
![logo](baup.png "opt title")

## A way to manage and backup your files

This project is a CLI utility to manage and backup certain files in your system across multiple devices. 
The CLI uses a configurable file to determine which files it needs to backup.

## Installation

To install the utility, follow the next instructions:

1. Clone the repository in your machine
2. Using cargo/rustc compile the project with the following command: `cargo build --release`
3. Create a symbolic link using: `sudo ln -s /path/to/executable /usr/bin/baup`

Alternatively, you can install the utility running the command: `sudo sh install.sh`

## Setup

Before starting to use the utility, you need to setup the auxiliar file.
You can create it manually, or run `baup edit` for creating and editing it.

An example of this file could be:
```
~/.config/helix/;helix

~/.config/rofi/config.rasi;rofi

/etc/X11/xorg.conf.d/30-touchpad.conf;system
/etc/systemd/logind.conf;system

~/.config/zathura/zathurarc;zathura

~/.config/espanso/;espanso
```
In this example, **all** the files in the ~/.config/helix/ directory will be copied to the helix subdirectory, the config.rasi file in the rofi subdirectory,...

**Important:** If you want to copy all the files of a directory, immediately before the ';' separator you have to add the '/' character. If you don't do this, the program will not work properly.

## Usage

The CLI Tool counts with subcommand to execute a certain action, this commands are:

### Import

The import command copies all the files specified in the backups file to the directory where this file is located.
The commands copies all the files recursively if the origin is a directory, and just the specified file if the origin is a file.

Moreover, the command counts with this options to optimize your workflow:

* **partial:** (-p,--partial <DIR>) Only imports the files where the subdirectory (the part after the ';') is equal to the DIR passed as argument
* **auto-commit:** (-c,--auto-commit) After importing the files, it creates a commit in which the message is a list of the changed files (if there are more than 3 files, it will show 3 and then ...) 

### Export

### Commit

### Push

### Pull

### Edit

### Diff

## Configuration



## Contributors
 - RodAlc24
 - ImanolCiganda
