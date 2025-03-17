#!/bin/bash

set -e  # Exit on any error

# Install script for ULID Tool
echo "Installing ULID Tool..."

# Build the release version
cargo build --release

# Create destination directory if it doesn't exist
mkdir -p $HOME/.local/bin

# Copy the binary
cp target/release/ulid $HOME/.local/bin/

# Set execute permissions
chmod +x $HOME/.local/bin/ulid

# Check if the directory is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
  echo "Adding $HOME/.local/bin to PATH in your shell profile"

  # Determine shell profile file
  if [ -n "$ZSH_VERSION" ]; then
    PROFILE=$HOME/.zshrc
  elif [ -n "$BASH_VERSION" ]; then
    PROFILE=$HOME/.bash_profile
    if [ ! -f "$PROFILE" ]; then
      PROFILE=$HOME/.profile
    fi
  else
    PROFILE=$HOME/.profile
  fi

  # Add to PATH
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> $PROFILE
  echo "Added to $PROFILE. Please restart your terminal or run:"
  echo "  source $PROFILE"
fi

echo "Installation complete! You can now use the 'ulid' command."
echo "Try running 'ulid --help' for usage information."