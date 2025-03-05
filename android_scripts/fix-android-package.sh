#!/bin/bash

# Make script exit on any error
set -e

echo "Diagnosing Android package structure issue..."

# Get the identifier from tauri.conf.json
IDENTIFIER=$(grep -o '"identifier": "[^"]*"' src-tauri/tauri.conf.json | cut -d'"' -f4)
echo "App identifier found in tauri.conf.json: $IDENTIFIER"

# Get Android package info
ANDROID_PACKAGE=$(grep -o '"package": "[^"]*"' src-tauri/tauri.conf.json | cut -d'"' -f4)
echo "Android package found in tauri.conf.json: $ANDROID_PACKAGE"

# Convert package name to directory structure (replace dots with slashes)
PACKAGE_DIR=${ANDROID_PACKAGE//\./\/}
echo "Expected package directory structure: $PACKAGE_DIR"

# Clean up Android build
echo "Removing previous Android build..."
rm -rf src-tauri/gen/android

# Reinitialize Android
echo "Reinitializing Android project..."
cargo tauri android init

# Check if directory was created
EXPECTED_DIR="src-tauri/gen/android/app/src/main/java/${PACKAGE_DIR}"
if [ -d "$EXPECTED_DIR" ]; then
  echo "✅ Success! Android package directory created at: $EXPECTED_DIR"
else
  echo "❌ Error: Android package directory not created at: $EXPECTED_DIR"
  
  # Create the directory structure if it doesn't exist
  echo "Creating directory structure manually..."
  mkdir -p "$EXPECTED_DIR"
  
  # Copy MainActivity.java if it exists elsewhere
  find src-tauri/gen/android -name "MainActivity.java" -exec cp {} "$EXPECTED_DIR/" \;
  
  echo "Directory structure created. Please check and adjust any files as needed."
fi

echo "Done."
