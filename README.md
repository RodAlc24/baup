# Project overview

This project is a utility to backup and manage certain files in your system. The utility consists of a bash script that reads a configurable file that tells the utility which files to import/export.
The path to the file is stored in the utility, so this file can be anywhere.

## Preparation

Before running the utility, it is necessary to create the file that tells the utility what files and directories to import/export.
This file doesn't need to be in any specific path, so it is up to the user to select which path is desired.

After creating the file, the user will need to enter the backupTool script and insert the path to the file in the FILE variable.

This file requires a specific syntax to work properly. This syntax consists of the path to the file or directory (with *) to backup and the relative folder in which to save those files in the backupsFile's directory.
For example:

``` 
/home/john/.config/nvim/init.vim;nvim
~/.config/kitty/kitty.conf;kitty
```

This will save init.vim in path/to/backupsFile/nvim/ and kitty.conf in path/to/backupsFile/kitty/

## Parameters

### Import

As the name says, import lets you import every file listed in the `backupsFile` to that folder. If there is a directory followed by *, the utility will save every file in every subdirectory following the same tree format.

### Export

On the contrary, export copies of the backup files to the paths listed in the `backupsFile`. You may need sudo for exporting certain files.

### Commit

Using git, commit creates a commit in the backupsFile's directory. The git repository needs to be created by the user.

For the commit's message, the utility will use the next parameter sent by the user.

### Push

Using git pushes all the commits that weren't pushed to github,...

### Diff

Finally, diff will show the changes that had been made to all the files in the `backupsFile`

## Note

Make sure that the `backupTool` script has the appropriate permissions to run. You can set the appropriate permissions with the following command:

``` 
chmod +x backupTool
``` 

Also, it is recommended that the user creates a symbolic link in the /usr/bin/ directory so the utility can be used from any directory. To do this, use the following command:

```
# Creates the symbolic link
sudo ln -s backupTool /usr/bin/backupTool
# Checks if the link has been created correctly
ls -l /usr/bin/backupTool
```

## Contributors

- RodAlc24
- ImanolCiganda
