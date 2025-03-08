#!/bin/bash

# Make script exit on any error
set -e

echo "Building Android TV app with absolute minimal Tauri configuration..."

# Set environment variables if not already set
if [ -z "$ANDROID_HOME" ]; then
  if [ -d "$HOME/Library/Android/sdk" ]; then
    export ANDROID_HOME="$HOME/Library/Android/sdk"
    echo "Set ANDROID_HOME=$ANDROID_HOME"
  else
    echo "❌ ANDROID_HOME not set and SDK not found"
    exit 1
  fi
fi

if [ -z "$NDK_HOME" ]; then
  if [ -d "$ANDROID_HOME/ndk" ]; then
    # Find an NDK version
    for dir in "$ANDROID_HOME/ndk"/*; do
      if [ -d "$dir" ]; then
        export NDK_HOME="$dir"
        echo "Set NDK_HOME=$NDK_HOME"
        break
      fi
    done
  elif [ -d "$ANDROID_HOME/ndk-bundle" ]; then
    export NDK_HOME="$ANDROID_HOME/ndk-bundle"
    echo "Set NDK_HOME=$NDK_HOME"
  else
    echo "❌ NDK_HOME not set and NDK not found"
    exit 1
  fi
fi

echo "Using:"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "NDK_HOME=$NDK_HOME"

# Backup original Cargo.toml if needed
if [ -f "src-tauri/Cargo.toml" ]; then
  if [ ! -f "src-tauri/Cargo.toml.orig" ]; then
    cp src-tauri/Cargo.toml src-tauri/Cargo.toml.orig
    echo "Created original backup at src-tauri/Cargo.toml.orig"
  fi
  cp src-tauri/Cargo.toml src-tauri/Cargo.toml.bak
  echo "Created backup at src-tauri/Cargo.toml.bak"
else
  echo "❌ Cargo.toml not found"
  exit 1
fi

# Clean previous build artifacts
echo "Cleaning previous build artifacts..."
rm -rf src-tauri/gen/android
rm -rf src-tauri/target/aarch64-linux-android

# Create an absolute minimal Cargo.toml known to work with Tauri 2.0.0-beta
cat > src-tauri/Cargo.toml << EOF
[package]
name = "blipty"
version = "1.0.0"
description = "A Tauri application"
authors = ["Mark"]
license = ""
repository = ""
edition = "2021"

[package.metadata.bundle]
identifier = "net.blipty.app"

[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2.0.0-beta" }

[dependencies]
tauri = "2.0.0-beta"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[features]
custom-protocol = ["tauri/custom-protocol"]
EOF

echo "Created minimal Cargo.toml"

# Try to build with absolute minimal configuration
echo "Building with minimal configuration..."
cd src-tauri
ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android init

# Update AndroidManifest.xml for TV
echo "Adding Android TV support to AndroidManifest.xml..."
MANIFEST_FILE="gen/android/app/src/main/AndroidManifest.xml"
if [ -f "$MANIFEST_FILE" ]; then
  # Create temporary files for macOS compatibility
  TEMP_FILE1=$(mktemp)
  TEMP_FILE2=$(mktemp)
  
  # Add TV features - first modification
  awk '{if ($0 ~ /<manifest xmlns:android="http:\/\/schemas.android.com\/apk\/res\/android">/) {print $0; print "    <uses-feature android:name=\"android.software.leanback\" android:required=\"false\" />"; print "    <uses-feature android:name=\"android.hardware.touchscreen\" android:required=\"false\" />";} else {print $0;}}' "$MANIFEST_FILE" > "$TEMP_FILE1"
  
  # Add TV launcher intent - second modification
  awk '{print $0; if ($0 ~ /<category android:name="android.intent.category.LAUNCHER" \/>/) {print "                <category android:name=\"android.intent.category.LEANBACK_LAUNCHER\" />";}}' "$TEMP_FILE1" > "$TEMP_FILE2"
  
  # Replace original file
  mv "$TEMP_FILE2" "$MANIFEST_FILE"
  rm -f "$TEMP_FILE1"
  
  echo "✅ AndroidManifest.xml updated for TV support"
else
  echo "⚠️ AndroidManifest.xml not found"
fi

# Build for Android
echo "Building Android app..."
ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android build --debug

# Check if build succeeded
if [ -f "gen/android/app/build/outputs/apk/debug/app-debug.apk" ]; then
  echo "✅ Build successful!"
  echo "APK located at: $(pwd)/gen/android/app/build/outputs/apk/debug/app-debug.apk"
  
  # Save working configuration
  cp Cargo.toml Cargo.toml.android-working
  echo "✅ Saved working configuration to Cargo.toml.android-working"
  
  # Create guide for incrementally adding back features
  cd ..
  cat > android-feature-guide.md << EOF
# Android Feature Integration Guide

This guide helps you incrementally add back features to your Tauri Android app.

## Working Minimal Build

A minimal build with TV support is now working! The APK is located at:
\`\`\`
src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk
\`\`\`

## Adding Features Back

Follow these steps to add back your features one by one:

1. Start with the minimal working Cargo.toml:
   \`\`\`bash
   cp src-tauri/Cargo.toml.android-working src-tauri/Cargo.toml
   \`\`\`

2. Add one dependency or feature at a time:
   \`\`\`bash
   # Edit Cargo.toml to add ONE dependency
   nano src-tauri/Cargo.toml
   
   # Then try building
   cd src-tauri
   ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android build --debug
   \`\`\`

3. If the build succeeds, save this working state:
   \`\`\`bash
   cp src-tauri/Cargo.toml src-tauri/Cargo.toml.android-working
   \`\`\`

4. If the build fails, revert and try a different dependency:
   \`\`\`bash
   cp src-tauri/Cargo.toml.android-working src-tauri/Cargo.toml
   \`\`\`

## Recommended Order for Adding Dependencies

1. Basic utilities: thiserror, log, simple_logger
2. Database: rusqlite with bundled feature
3. Async runtime: tokio
4. Network (with rustls): reqwest with \`default-features = false, features = ["json", "rustls-tls"]\`

## Testing on Android TV

1. Install your app:
   \`\`\`bash
   adb install -r src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk
   \`\`\`

2. Use the remote control script:
   \`\`\`bash
   ./tv-remote.sh [command]
   \`\`\`
EOF

  echo "Created guide for adding back features at android-feature-guide.md"
  
  # Ask to install
  if adb devices | grep -q "device$"; then
    echo ""
    echo "Android device detected. Install now? (y/n)"
    read -r answer
    if [ "$answer" = "y" ]; then
      echo "Installing app..."
      adb install -r "src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk"
      echo "App installed!"
    fi
  else
    echo "No Android device detected."
  fi
else
  echo "❌ Build failed"
  
  # Restore backup
  cd ..
  echo "Restoring Cargo.toml backup..."
  cp src-tauri/Cargo.toml.bak src-tauri/Cargo.toml
  echo "Backup restored"
  
  # Check if tauri is installed correctly
  echo "Checking Tauri CLI version..."
  cargo tauri --version
  
  echo "You might try:"
  echo "1. Running 'cargo update' to update dependencies"
  echo "2. Check if your Tauri version matches the documentation"
  echo "3. Try creating a new minimal Tauri project with 'cargo tauri init' in a new directory"
fi
