#!/bin/bash

# Define colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# read list of file paths and names from files.txt
while IFS=';' read -r file_path file_name; do
    file_all=$(eval echo "$file_path/$file_name")

  # check if file exists in backupFiles directory
  if [ -f "./backupFiles/$file_name" ]; then
    # create directory path if it doesn't exist
    mkdir -p $(eval echo "$file_path")
    # copy file to specified path
    if rsync --itemize-changes --update "./backupFiles/$file_name" "$file_all" | grep -q ">f"; then
      echo -e " [${GREEN}OK${NC}] Copied $file_name to $file_path"
    else 
      echo -e " [${GREEN}OK${NC}] $file_name is up to date"
    fi
  else
    echo -e " [${RED}ERROR${NC}] $file_name does not exist in ./backupFiles/"
  fi
done < ./backupFiles/files.txt

