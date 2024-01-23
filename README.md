# BAUP
![logo](baup.png "opt title")

## A way to manage and backup your files

This project is a utility to manage and backup certain files in your system across multiple devices. 
The project uses a configurable file to determine which files it needs to backup.

## Installation

To install the utility, follow the next instructions:

1. Clone the repository in your machine
2. Using cargo/rustc compile the project with the following command: ´cargo build --release´
3. Create a symbolic link using: ´sudo ln -s /path/to/executable /usr/bin/baup´

Alternatively, you can install the utility running the command: ´sudo sh install.sh´

## Setup

Before starting to use the utility, you need to setup the auxiliar file.
You can create it manually, or run ´baup edit´ for creating and editing it.

This file requires a very specific syntax for working properly
