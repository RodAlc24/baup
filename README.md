# Project overview


This project consists of two bash scripts and a directory containing some files. The directory `backupFiles/` contains the files that will be imported and exported and the `files.txt` file that contains the path and filename of each file that will be exported or imported. The `exportFiles.sh` script exports the files to their specified path, and the `importFiles.sh` script imports the files from their specified path.

## Usage

### Preparing `backupFiles` and `files.txt`


Before running the scripts, you need to create a `backupFiles` directory and a `files.txt` file inside it. 

```
.
├── README.md
├── backupFiles
│   └── files.txt
├── exportFiles.sh
└── importFiles.sh
```

`files.txt` specifies the path and filename for each file you want to export or import. Each line in the file should be in the format `relative/or/absolute/path;file_name`.
For example:

``` 
/home/john/.config/nvim;init.vim
~/.config/kitty;kitty.conf
```


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

## Dependencies

`rsync`

## Note

Make sure that the `exportFiles.sh` and `importFiles.sh` scripts have the appropriate permissions to run. You can set the appropriate permissions with the following command:

``` 
chmod +x exportFiles.sh importFiles.sh
``` 

## Buckup repo

The `backupFiles/` directory may be a git repository.
