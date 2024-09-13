# 0.2.2

* The diff command stops when it detects changes as it did in the bash version
* The log now always remembers the command used when the error was found

# 0.2.1

* Removed from the utility the commit, push and pull commands
* Added a git command to the utility. The git command allows the user to call any git subcommand it wants

# 0.2.0

* Rewrite the program in Rust
* Changed the way you call the commands (Example: In version 0.1.0 you will use `baup -i` for importing, whereas in this version you will use `baup import`)
* Added the partial, zip and auto-commit flags for the import command
* Added the partial flag to the export command
* Commit, Push and Pull command support all arguments that git does
* Added the partial flag to the diff command
* Created the clear command for deleting files in the backups directory (it supports the partial flag)

# 0.1.0

TODO
