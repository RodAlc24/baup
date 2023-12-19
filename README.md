# Project overview

This project is a utility to backup and manage certain files in your system. The utility consists of a bash script that reads a configurable file that tells the utility which files to import/export.
The path to the file is stored in the utility, so this file can be anywhere.

## Usage

### Preparing `backupsFile`

Before running the utility, it is necessary to create the file that tells the utility what files and directories to import/export.
This file doesn't need to be in any specific path, so it is up to the user to select which path is the desired.

After creating the file, the user will need to enter the backupTool script and insert the path to the file in the FILE variable.

This file requires a specific syntaxis to work properly. This syntaxis consists of:


``` 
/home/john/.config/nvim/init.vim;nvim
~/.config/kitty/kitty.conf;kitty
```

This will save init.vim in ./backupFiles/nvim and kitty.conf in ./backupFiles/kitty

### Exporting files

To export files from the `backupFiles/` directory to their corresponding paths specified in `files.txt`, run the following command:

``` 
./exportFiles.sh 
```

This will copy the files from `backupFiles/` to the paths specified in `files.txt`. If the specified path does not exist, it will be created.

### Importing files

To import files from their specified paths in `files.txt` to the `backupFiles/` directory, run the following command:

```
./importFiles.sh
``` 

This will copy the files from their specified paths in `files.txt` to the `backupFiles/` directory. If the `backupFiles/` directory does not exist, it will be created.

## Note

Make sure that the `exportFiles.sh` and `importFiles.sh` scripts have the appropriate permissions to run. You can set the appropriate permissions with the following command:

``` 
chmod +x exportFiles.sh importFiles.sh
``` 

## Backup repo

The `backupFiles/` directory may be a git repository.

## Contributors

- RodAlc24
- ImanolCiganda
