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

This command has the oposite function as import, copying the files from the backups folder to their original locations.
The export command counts, as import did, with the partial (-p,--partial) option for exporting only one part of the files.

### Commit

Using git, this command commits the changes made in the backups repository.
Any options that you write after baup will be passed as arguments to the git commit command.

For example, if you ran `baup commit -m "Test"`, the program will `git add .` and `git commit -m "Test"`

### Push

As the commit command did, push uses git to push all the commits made in the repository.
Also, the push command will pass any arguments to the git command.

### Pull

This command works exactly the same as push or commit, but for pulling any changes to the local repository from the remote repository.

### Edit

The edit command will (using your default editor saved in $EDITOR) open the backups file for you to change it in any way you need.

### Diff

## Configuration

The program can be configured using a configuration file. This file will be located in `~/.config/baup/config.toml`.
The values that can be changed are the following:

* **path** -> The path to the backups file (`~/.baup/files.txt` for default)
* **auto_commit** -> Works exactly the same as the auto-commit option for the import command (false for default)
* **import_hook** -> A script that will be execute it before any files are imported (None for default)
* **import_hook** -> A script that will be execute it before any files are exported (None for default)

An example configuration file can be:
```
path = "~/Documents/baup/files.txt"

[hooks]
import_hook="~/.config/baup/hooks/import.sh"
```
As you can see in the example, you don't need to set all the values in the config file for it to work

## Contributors
 - RodAlc24
 - ImanolCiganda
