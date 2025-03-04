#!/bin/bash

# Script to build and deploy the app to an Android TV device or emulator

# Configuration
APK_PATH="app/build/outputs/apk/debug/app-debug.apk"
PACKAGE_NAME="com.yourcompany.iptv"
MAIN_ACTIVITY=".MainActivity"

# Make sure we're in the project root
if [ ! -f "gradlew" ]; then
  echo "Error: Run this script from the project root directory!"
  exit 1
fi

# Build the APK
echo "Building the APK..."
./gradlew assembleDebug

if [ $? -ne 0 ]; then
  echo "Build failed!"
  exit 1
fi

# Check for connected devices
devices=$(adb devices | grep -v "List" | grep -v "^$" | wc -l)

if [ "$devices" -eq 0 ]; then
  echo "No devices connected. Please connect an Android TV device or start an emulator."
  
  # Ask if user wants to start the emulator
  read -p "Do you want to start an Android TV emulator? (y/n): " start_emulator
  
  if [ "$start_emulator" == "y" ]; then
    # List available TV emulators
    echo "Available Android TV emulators:"
    emulator -list-avds | grep -i tv
    
    # Ask for emulator name
    read -p "Enter emulator name to launch: " emulator_name
    
    # Start emulator in background
    echo "Starting emulator..."
    emulator -avd "$emulator_name" &
    
    # Wait for emulator to boot
    echo "Waiting for emulator to boot..."
    adb wait-for-device
    sleep 10
  else
    exit 1
  fi
else
  echo "Device connected. Continuing..."
fi

# Install the APK
echo "Installing the APK..."
adb install -r "$APK_PATH"

if [ $? -ne 0 ]; then
  echo "Installation failed!"
  exit 1
fi

# Launch the app
echo "Launching the app..."
adb shell am start -n "$PACKAGE_NAME/$PACKAGE_NAME$MAIN_ACTIVITY"

# Monitor logcat
echo "Monitoring logcat output. Press Ctrl+C to stop."
adb logcat | grep "$PACKAGE_NAME"
