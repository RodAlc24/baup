# 0.2.2

* Added interactive flag to the diff command, that stops waiting for user input when a change is found
* The log now always remembers the command used when the error was found
* The `nano` editor now has preference over `vim` in the edit command

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

Initial version of the program written in Bash. 
It is used by calling baup and the flag(s) of the command you want to use.
For example, `baup -i -c "New changes" -p` will import, create a commit called "New changes" and push using git
