#!/bin/bash

# Define colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# read list of file paths and names from files.txt
while IFS=';' read -r file_path file_name; do
    file_all=$(eval echo "$file_path/$file_name")

  # check if file exists in specified path
  if [ -f "$file_all" ]; then
    # create backupFiles directory if it doesn't exist
    mkdir -p ./backupFiles
    # copy file to backupFiles directory if it has been modified
    if rsync --itemize-changes --update "$file_all" ./backupFiles/ | grep -q ">f"; then
      echo -e " [${GREEN}OK${NC}] Copied $file_name to ./backupFiles/"
    else 
      echo -e " [${GREEN}OK${NC}] $file_name is up to date"
    fi
  else
    echo -e " [${RED}ERROR${NC}] $file_name does not exist in $file_path"
  fi
done < ./backupFiles/files.txt

