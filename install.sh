#!/bin/bash
if [ "$EUID" -ne 0 ]
 then echo "Please run as root"
 exit
else
  SCRIPT_DIR=$(dirname $(realpath "$0"))
  # Create executable
  cargo build --release
  BINARY_DIR="$SCRIPT_DIR/target/release/baup"
  echo "[0] Create symbolic link"
  echo "[1] Copy the executable"
  read -p "Select Option: " SEL
  if [ $SEL -eq 0 ]; then
    sudo ln -s $BINARY_DIR /usr/bin/baup
    ls -l /usr/bin/baup
  elif [ $SEL -eq 1 ]; then
    cp $BINARY_DIR /usr/bin/baup
  else
    echo "Invalid option. Please run the script again"
  fi
fi

