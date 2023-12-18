#!/bin/bash

# File with the files to backup
FILE="/home/imanol/.config/backUp/backupFiles/files.txt"

# Define colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

function import_files(){
# Read list of file paths and names from files.txt
while IFS=';' read -r file file_folder; do
    file_all=$(eval echo "$file")
    directory_all=$(eval echo "$file_folder")
    file_name=$(basename "$file_all")
  
  # Check if file exists in specified path
  if [ -f "$file_all" ]; then
    # Create backupFiles directory if it doesn't exist
    mkdir -p ./backupFiles
    # Create directory structure in backupFiles directory
    mkdir -p "./backupFiles/$directory_all" 
    # Copy file to backupFiles directory
    if cp "$file_all" "./backupFiles/$directory_all"; then
      echo -e " [${GREEN}OK${NC}] Copied $file_name to ./backupFiles/$directory_all"
    else 
      echo -e " [${GREEN}OK${NC}] $file_name is up to date"
    fi
  else
    echo -e " [${RED}ERROR${NC}] $file_name does not exist"
  fi
done < "$1"
}

function export_files(){
# Read list of file paths and names from files.txt
while IFS=';' read -r file file_folder; do
    file_all=$(eval echo "$file")
    directory_all=$(eval echo "$file_folder")
    file_path=$(dirname "$file")
    file_name=$(basename "$file_all")

  # Check if file exists in backupFiles directory
  if [ -f "./backupFiles/$directory_all/$file_name" ]; then
    # Create directory path if it doesn't exist
    mkdir -p $(eval echo "$file_path")
    # Copy file to specified path
    if cp "./backupFiles/$directory_all/$file_name" "$file_all"; then
      echo -e " [${GREEN}OK${NC}] Copied $file_name to $file_path"
    fi
  else
    echo -e " [${RED}ERROR${NC}] $file_name does not exist in ./backupFiles/$directory_all"
  fi
done < "$1"
}

function commit(){
  # Detects folder of the backups	
  backup_folder=$(dirname "$1")
  # Moves to that folder
  cd "$backup_folder"
  if [ $? -eq 0 ]; then
    commit_message="$2"
    # Does the commit and push
    git add .
    git commit -m "$commit_message"
    git push
  fi
}

function edit(){
  $EDITOR "$1"
}

function help(){
  echo "Help"  
}

function check_diff(){
while IFS=';' read -r file file_folder; do
    file_all=$(eval echo "$file")
    directory_all=$(eval echo "$file_folder")
    file_path=$(dirname "$file")
    file_name=$(basename "$file_all")

  # Check if file exists in backupFiles directory
  if [ -f "./backupFiles/$directory_all/$file_name" ]; then
    # Checks diff in file
    diff -u --color "$file_all" "./backupFiles/$directory_all/$file_name"
    if [ $? -eq 0 ]; then
      echo -e " [${GREEN}OK${NC}] There are no changes in: $file"
    fi
      read -n 1 -p "Press 'q' to exit ; Press any other key to compare next file " key < /dev/tty  
      if [[ ${key,,} == 'q' ]]; then
        return
      fi
  else
    echo -e " [${RED}ERROR${NC}] $file_name does not exist in ./backupFiles/$directory_all"
  fi
done < "$1"
}

if [ "$#" -eq 0 ]; then
  help
elif [ "$#" -eq 1 ] && [ $1 == "edit" ]; then
  edit $FILE
else
  for (( i=1; i<=$#; i++)); do
    j=$((i+1))
    case ${!i} in
      "-h" | "--help")
        help
        break
        ;;
      "-i" | "--import")
        import_files $FILE
        ;;
      "-e" | "--export")
        export_files $FILE
        ;;
      "-c" | "--commit")
        if [ $j -le $# ]; then
          commit $FILE "${!j}"
          ((i++))
        else
          echo -e "[${RED}ERROR${NC}] No commit message passed"
          break
        fi
        ;;
      "-d" | "--diff")
        check_diff $FILE
        ;;
      *)
        echo "Argumento: ${!i}"
        ;;
    esac
  done
fi


