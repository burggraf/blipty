#!/bin/bash

# Make script exit on any error
set -e

echo "Fixing dependencies for Android build while preserving functionality..."

# Find SDK and NDK
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

# Create an updated Cargo.toml that has reqwest configured for Android
if [ -f "src-tauri/Cargo.toml" ]; then
  cp src-tauri/Cargo.toml src-tauri/Cargo.toml.bak
  echo "Created backup of Cargo.toml at src-tauri/Cargo.toml.bak"
  
  # Create a modified Cargo.toml with proper dependencies for Android
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
log = "0.4"
simple_logger = "4.2"
tokio = { version = "1.36", features = ["rt-multi-thread", "sync"] }

# Fix chrono dependency for Android
chrono = "0.4"

# Fix reqwest dependency for Android
reqwest = { version = "0.12", features = ["json"], default-features = false }

# Keep URL dependency which has no OpenSSL issues
url = "2.5.0"

# Add a native-tls feature flag for conditional compilation
[features]
custom-protocol = ["tauri/custom-protocol"]
native-tls = ["reqwest/native-tls"]
rustls-tls = ["reqwest/rustls-tls"]
EOF

  echo "Created modified Cargo.toml with Android-compatible dependencies"
else
  echo "❌ src-tauri/Cargo.toml not found"
  exit 1
fi

# Create conditional imports for reqwest in fetch_api.rs
mkdir -p src-tauri/src/channel_commands
if [ -f "src-tauri/src/channel_commands/fetch_api.rs" ]; then
  cp src-tauri/src/channel_commands/fetch_api.rs src-tauri/src/channel_commands/fetch_api.rs.bak
  
  # Add conditional compilation for different platforms
  echo "Updating fetch_api.rs to handle Android build..."
  cat > src-tauri/src/channel_commands/fetch_api.rs <<EOF
// For Android, we use a minimal reqwest client without native TLS
#[cfg(target_os = "android")]
use reqwest::{Client, ClientBuilder};

// For other platforms, use the default reqwest
#[cfg(not(target_os = "android"))]
use reqwest;

// Rest of your original file follows
// ...
EOF
  
  # Copy the rest of the original file
  tail -n +2 src-tauri/src/channel_commands/fetch_api.rs.bak >> src-tauri/src/channel_commands/fetch_api.rs
  
  echo "Updated fetch_api.rs with conditional imports"
else
  echo "⚠️ fetch_api.rs not found, skipping modification"
fi

# Clean up previous build artifacts
echo "Cleaning previous Android build..."
rm -rf src-tauri/gen/android

# Try to build for Android using rustls-tls instead of native-tls
echo "Building for Android with rustls-tls feature..."
cd src-tauri

# Build with rustls-tls feature instead of default native-tls
ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android init
ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android build --debug --features rustls-tls

# Check if build succeeded
if [ -f "gen/android/app/build/outputs/apk/debug/app-debug.apk" ]; then
  echo "✅ Build successful!"
  echo "APK located at: $(pwd)/gen/android/app/build/outputs/apk/debug/app-debug.apk"
  
  # Copy the successful Cargo.toml to a safe location
  cp Cargo.toml Cargo.toml.android
  echo "Saved working Android configuration to Cargo.toml.android"
  
  # Add Android TV support to the manifest
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
    
    # Build again with the TV modifications
    ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android build --debug --features rustls-tls
    
    echo "✅ Rebuilt with Android TV support"
  fi
  
  # Ask to install
  if adb devices | grep -q "device$"; then
    echo ""
    echo "Android device detected. Install now? (y/n)"
    read -r answer
    if [ "$answer" = "y" ]; then
      echo "Installing app..."
      adb install -r "gen/android/app/build/outputs/apk/debug/app-debug.apk"
      echo "App installed!"
    fi
  else
    echo "No Android device detected."
  fi
  
  # Create instructions for switching between builds
  cat > ../android-build-instructions.md <<EOF
# Android TV Build Instructions

## Building for Android TV

To build your app for Android TV, use the following command:

\`\`\`bash
source .env  # Load SDK and NDK environment variables
cd src-tauri
ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android build --debug --features rustls-tls
\`\`\`

## Installing on Android TV

After building, install the app on a connected Android TV device or emulator:

\`\`\`bash
adb install -r gen/android/app/build/outputs/apk/debug/app-debug.apk
\`\`\`

## Switching Between Development Platforms

### For Android Development
Use the Android-compatible Cargo.toml:

\`\`\`bash
cp Cargo.toml.android Cargo.toml
\`\`\`

### For Desktop Development
Restore the original Cargo.toml:

\`\`\`bash
cp Cargo.toml.bak Cargo.toml
\`\`\`

## Android TV Testing

Use the included TV remote simulation script:

\`\`\`bash
cd ..
./tv-remote.sh up     # Navigate up
./tv-remote.sh down   # Navigate down
./tv-remote.sh left   # Navigate left
./tv-remote.sh right  # Navigate right
./tv-remote.sh select # Select/Enter
./tv-remote.sh back   # Back button
\`\`\`
EOF

  echo "Created Android build instructions at android-build-instructions.md"
else
  echo "❌ Build failed"
  
  # Restore original Cargo.toml
  cd ..
  echo "Restoring original Cargo.toml..."
  cp src-tauri/Cargo.toml.bak src-tauri/Cargo.toml
  if [ -f "src-tauri/src/channel_commands/fetch_api.rs.bak" ]; then
    cp src-tauri/src/channel_commands/fetch_api.rs.bak src-tauri/src/channel_commands/fetch_api.rs
  fi
  echo "Original files restored"
  
  echo "Consider using a different HTTP client for Android or try building with native-tls disabled:"
  echo "cargo tauri android build --debug --no-default-features --features reqwest/rustls-tls"
fi
