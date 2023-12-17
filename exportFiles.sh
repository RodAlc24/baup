#!/bin/bash

# Define colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# read list of file paths and names from files.txt
while IFS=';' read -r file file_directory; do
    file_all=$(eval echo "$file")
    directory_all=$(eval echo "$file_directory")
    file_path=$(dirname "$file")
    file_name=$(basename "$file_all")

  # check if file exists in backupFiles directory
  if [ -f "./backupFiles/$directory_all/$file_name" ]; then
    # create directory path if it doesn't exist
    mkdir -p $(eval echo "$file_path")
    # copy file to specified path
    cp "./backupFiles/$directory_all/$file_name" "$file_all"
    echo -e " [${GREEN}OK${NC}] Copied $file_name to $file_path"
  else
    echo -e " [${RED}ERROR${NC}] $file_name does not exist in ./backupFiles/$directory_all"
  fi
done < ./backupFiles/files.txt

