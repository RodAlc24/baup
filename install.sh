#!/bin/bash
if [ "$EUID" -ne 0 ]
 then echo "Please run as root"
 exit
else
  SCRIPT_DIR=$(dirname $(realpath "$0"))
  # Creates the binary
  cargo build --release
  BINARY_DIR="$SCRIPT_DIR/target/release/baup"
  # Creates the symbolic link
  sudo ln -s $BINARY_DIR /usr/bin/baup2
  ls -l /usr/bin/baup2
fi

