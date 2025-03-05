#!/bin/bash

# Make script exit on any error
set -e

echo "Building Android TV app with correct features..."

# Find SDK
if [ -z "$ANDROID_HOME" ]; then
  if [ -d "$HOME/Library/Android/sdk" ]; then
    export ANDROID_HOME="$HOME/Library/Android/sdk"
  elif [ -d "/Applications/Android Studio.app/Contents/sdk" ]; then
    export ANDROID_HOME="/Applications/Android Studio.app/Contents/sdk"
  fi
  
  if [ -z "$ANDROID_HOME" ]; then
    echo "❌ ANDROID_HOME not set and SDK not found in common locations"
    exit 1
  fi
  
  echo "✅ Found Android SDK at: $ANDROID_HOME"
fi

# Find NDK
if [ -z "$NDK_HOME" ]; then
  if [ -d "$ANDROID_HOME/ndk" ]; then
    # Find latest NDK
    LATEST_NDK=$(find "$ANDROID_HOME/ndk" -maxdepth 1 -type d | sort -r | head -n 2 | tail -n 1)
    if [ -n "$LATEST_NDK" ]; then
      export NDK_HOME="$LATEST_NDK"
      echo "✅ Found NDK at: $NDK_HOME"
    fi
  fi
  
  if [ -z "$NDK_HOME" ] && [ -d "$ANDROID_HOME/ndk-bundle" ]; then
    export NDK_HOME="$ANDROID_HOME/ndk-bundle"
    echo "✅ Found NDK at: $NDK_HOME"
  fi
  
  if [ -z "$NDK_HOME" ]; then
    echo "❌ NDK_HOME not set and NDK not found in common locations"
    exit 1
  fi
fi

echo "Using:"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "NDK_HOME=$NDK_HOME"

# Reset Android project
echo "Cleaning previous Android build..."
rm -rf src-tauri/gen/android

# Initialize Android project
echo "Initializing Android project..."
cd src-tauri
cargo tauri android init

# Update AndroidManifest.xml for TV
echo "Adding TV support to manifest..."
MANIFEST="gen/android/app/src/main/AndroidManifest.xml"
if [ -f "$MANIFEST" ]; then
  # Add TV features if they don't exist
  if ! grep -q "android.software.leanback" "$MANIFEST"; then
    sed -i.bak 's#<manifest xmlns:android="http://schemas.android.com/apk/res/android">#<manifest xmlns:android="http://schemas.android.com/apk/res/android">\n    <uses-feature android:name="android.software.leanback" android:required="false" />\n    <uses-feature android:name="android.hardware.touchscreen" android:required="false" />#' "$MANIFEST"
    echo "✅ Added TV features to manifest"
  fi
  
  # Add Leanback launcher intent if it doesn't exist
  if ! grep -q "LEANBACK_LAUNCHER" "$MANIFEST"; then
    sed -i.bak 's#<category android:name="android.intent.category.LAUNCHER" />#<category android:name="android.intent.category.LAUNCHER" />\n                <category android:name="android.intent.category.LEANBACK_LAUNCHER" />#' "$MANIFEST"
    echo "✅ Added Leanback launcher to manifest"
  fi
else
  echo "❌ AndroidManifest.xml not found"
fi

# Build with correct environment variables
echo "Building Android app..."
ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android build --debug

# Check if build succeeded
if [ -f "gen/android/app/build/outputs/apk/debug/app-debug.apk" ]; then
  echo "✅ Build successful!"
  echo "APK located at: $(pwd)/gen/android/app/build/outputs/apk/debug/app-debug.apk"
  
  # Ask to install if device connected
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
else
  echo "❌ Build failed"
fi
