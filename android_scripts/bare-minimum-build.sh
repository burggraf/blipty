#!/bin/bash

# Make script exit on any error
set -e

echo "Building Android TV app with bare minimum configuration..."

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

# Clean up previous build
echo "Cleaning previous Android build..."
rm -rf src-tauri/gen/android

# Check if Cargo.toml exists and create backup
if [ -f "src-tauri/Cargo.toml" ]; then
  cp src-tauri/Cargo.toml src-tauri/Cargo.toml.bak
  echo "Created backup of Cargo.toml at src-tauri/Cargo.toml.bak"
  
  # Create a bare minimum Cargo.toml with no features
  cat > src-tauri/Cargo.toml <<EOF
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
tauri = { version = "2.0.0-beta" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rusqlite = { version = "0.30.0", features = ["bundled"] }
thiserror = "1.0"
chrono = "0.4"
log = "0.4"
simple_logger = "4.2"
reqwest = { version = "0.12", features = ["json"] }
url = "2.5.0"
tokio = { version = "1.36", features = ["rt-multi-thread", "sync"] }

[features]
custom-protocol = ["tauri/custom-protocol"]
EOF

  echo "Created bare minimum Cargo.toml"
else
  echo "❌ src-tauri/Cargo.toml not found"
  exit 1
fi

# Initialize Android project
echo "Initializing Android project..."
cd src-tauri
cargo tauri android init

echo "Adding Android TV support to AndroidManifest.xml..."
MANIFEST_FILE="gen/android/app/src/main/AndroidManifest.xml"
if [ -f "$MANIFEST_FILE" ]; then
  # Create temporary files for macOS compatibility
  TEMP_FILE1=$(mktemp)
  TEMP_FILE2=$(mktemp)
  
  # Add TV features
  awk '{if ($0 ~ /<manifest xmlns:android="http:\/\/schemas.android.com\/apk\/res\/android">/) {print $0; print "    <uses-feature android:name=\"android.software.leanback\" android:required=\"false\" />"; print "    <uses-feature android:name=\"android.hardware.touchscreen\" android:required=\"false\" />";} else {print $0;}}' "$MANIFEST_FILE" > "$TEMP_FILE1"
  
  # Add TV launcher intent
  awk '{print $0; if ($0 ~ /<category android:name="android.intent.category.LAUNCHER" \/>/) {print "                <category android:name=\"android.intent.category.LEANBACK_LAUNCHER\" />";}}' "$TEMP_FILE1" > "$TEMP_FILE2"
  
  # Replace original file
  mv "$TEMP_FILE2" "$MANIFEST_FILE"
  rm -f "$TEMP_FILE1"
  
  echo "✅ AndroidManifest.xml updated for TV support"
else
  echo "⚠️ AndroidManifest.xml not found"
fi

# Add a banner image for Android TV
DRAWABLE_DIR="gen/android/app/src/main/res/drawable"
if [ ! -d "$DRAWABLE_DIR" ]; then
  mkdir -p "$DRAWABLE_DIR"
fi

cat > "$DRAWABLE_DIR/banner.xml" <<EOF
<?xml version="1.0" encoding="utf-8"?>
<layer-list xmlns:android="http://schemas.android.com/apk/res/android">
    <item android:drawable="@color/ic_launcher_background" />
    <item
        android:drawable="@mipmap/ic_launcher_foreground"
        android:gravity="center" />
</layer-list>
EOF

echo "✅ Created TV banner image"

# Build with direct environment variables
echo "Building Android TV app..."
ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android build --debug

# Check if build succeeded
if [ -f "gen/android/app/build/outputs/apk/debug/app-debug.apk" ]; then
  echo "✅ Build successful!"
  echo "APK located at: $(pwd)/gen/android/app/build/outputs/apk/debug/app-debug.apk"
  
  # Restore original Cargo.toml
  cd ..
  echo "Restoring original Cargo.toml..."
  cp src-tauri/Cargo.toml.bak src-tauri/Cargo.toml
  echo "Original Cargo.toml restored"
  
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
  
  echo ""
  echo "Next steps:"
  echo "1. Run './setup-tv-emulator.sh' to set up an Android TV emulator"
  echo "2. Start the emulator with 'emulator -avd Android_TV'"
  echo "3. Install the app with 'adb install src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk'"
  echo "4. Use './tv-remote.sh' to simulate TV remote controls"
else
  echo "❌ Build failed"
  
  # Restore original Cargo.toml
  cd ..
  echo "Restoring original Cargo.toml..."
  cp src-tauri/Cargo.toml.bak src-tauri/Cargo.toml
  echo "Original Cargo.toml restored"
  
  # Provide debugging information
  echo ""
  echo "Debug information:"
  echo "1. Check that you have the latest NDK installed"
  echo "2. Verify that Tauri 2.0.0-beta supports Android"
  echo "3. Consider running 'cargo update' to update dependencies"
  echo "4. Check the Tauri documentation for Android support"
fi
