#!/bin/bash

# Make script exit on any error
set -e

echo "Building Android TV app with explicit environment variable setup..."

# Locate Android SDK
POSSIBLE_SDK_LOCATIONS=(
  "$HOME/Library/Android/sdk"
  "/Applications/Android Studio.app/Contents/sdk"
  "$HOME/Android/Sdk"
)

for location in "${POSSIBLE_SDK_LOCATIONS[@]}"; do
  if [ -d "$location" ]; then
    export ANDROID_HOME="$location"
    break
  fi
done

if [ -z "$ANDROID_HOME" ]; then
  echo "❌ Android SDK not found! Please install Android Studio."
  exit 1
fi

echo "✅ Found Android SDK at: $ANDROID_HOME"

# Find NDK within SDK directory
if [ -d "$ANDROID_HOME/ndk" ]; then
  # Get the available NDK versions by looking for directories within the ndk folder
  NDK_VERSIONS=()
  for dir in "$ANDROID_HOME/ndk"/*/; do
    if [ -d "$dir" ] && [ -f "$dir/source.properties" ]; then
      NDK_VERSIONS+=("$dir")
    fi
  done
  
  if [ ${#NDK_VERSIONS[@]} -gt 0 ]; then
    # Use the first valid NDK found (usually the latest)
    export ANDROID_NDK_HOME="${NDK_VERSIONS[0]}"
    export NDK_HOME="${NDK_VERSIONS[0]}"
    echo "✅ Found Android NDK at: $NDK_HOME"
  else
    echo "❌ No valid NDK versions found in $ANDROID_HOME/ndk"
    exit 1
  fi
else
  # Try to find NDK in alternative locations
  if [ -d "$ANDROID_HOME/ndk-bundle" ]; then
    export ANDROID_NDK_HOME="$ANDROID_HOME/ndk-bundle"
    export NDK_HOME="$ANDROID_HOME/ndk-bundle"
    echo "✅ Found Android NDK at: $NDK_HOME"
  else
    echo "❌ Android NDK not found. Please install it through Android Studio SDK Manager."
    echo "1. Open Android Studio"
    echo "2. Go to Tools > SDK Manager"
    echo "3. Select the 'SDK Tools' tab"
    echo "4. Check 'NDK (Side by side)' and click Apply/OK"
    exit 1
  fi
fi

# Set PATH to include Android tools
export PATH="$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/tools/bin:$ANDROID_HOME/platform-tools"

# Print environment for debugging
echo "Using environment variables:"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "ANDROID_NDK_HOME=$ANDROID_NDK_HOME"
echo "NDK_HOME=$NDK_HOME"

# Setup OpenSSL for Android
export OPENSSL_DIR="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr"
export OPENSSL_INCLUDE_DIR="$OPENSSL_DIR/include"
export OPENSSL_LIB_DIR="$OPENSSL_DIR/lib/aarch64-linux-android"

# Go to src-tauri directory
cd "$(dirname "$0")/src-tauri"

# Check for existing Android directory and prompt for cleaning
if [ -d "gen/android" ]; then
  echo "Found existing Android build directory."
  echo "Do you want to clean it and start fresh? (y/n)"
  read -r answer
  if [[ "$answer" == "y" ]]; then
    echo "Cleaning existing Android build..."
    rm -rf gen/android
  fi
fi

# Initialize Android project if needed
if [ ! -d "gen/android" ]; then
  echo "Initializing Android project..."
  # Ensure the command uses the correct NDK path
  RUST_BACKTRACE=1 cargo tauri android init
fi

# Verify Android project was created
if [ ! -d "gen/android" ]; then
  echo "❌ Failed to initialize Android project!"
  exit 1
fi

# Apply Android TV manifest changes
MANIFEST_FILE="gen/android/app/src/main/AndroidManifest.xml"

if [ -f "$MANIFEST_FILE" ]; then
  echo "Adding Android TV support to manifest..."
  
  # Backup the original file
  cp "$MANIFEST_FILE" "${MANIFEST_FILE}.bak"
  
  # Add TV features to the manifest
  awk '
  /<manifest/,/<application/ {
    if ($0 ~ /<application/) {
      print "    <!-- Android TV support -->";
      print "    <uses-feature android:name=\"android.software.leanback\" android:required=\"false\" />";
      print "    <uses-feature android:name=\"android.hardware.touchscreen\" android:required=\"false\" />";
    }
    print;
    next;
  }
  /<activity[^>]*android:name=".MainActivity"/,/<\/activity>/ {
    if ($0 ~ /<\/activity>/) {
      print "            <intent-filter>";
      print "                <action android:name=\"android.intent.action.MAIN\" />";
      print "                <category android:name=\"android.intent.category.LEANBACK_LAUNCHER\" />";
      print "            </intent-filter>";
    }
    print;
    next;
  }
  { print; }
  ' "${MANIFEST_FILE}.bak" > "$MANIFEST_FILE"
  
  echo "✅ Updated AndroidManifest.xml for TV support"
else
  echo "❌ AndroidManifest.xml not found! Make sure Android initialization succeeded."
  exit 1
fi

# Check if TV configuration file exists and update it
TV_CONFIG_FILE="../android/tv-config.js"
if [ -f "$TV_CONFIG_FILE" ]; then
  echo "Found Android TV configuration file, applying TV-specific settings..."
  # Here you would add any TV-specific configuration copying logic
fi

# Build the app
echo "Building Android app..."
cargo tauri android build --debug

# Check if build succeeded
if [ -f "gen/android/app/build/outputs/apk/debug/app-debug.apk" ]; then
  echo "✅ Build successful!"
  echo "APK located at: gen/android/app/build/outputs/apk/debug/app-debug.apk"
  
  # Check if a device is connected
  if adb devices | grep -q "device$"; then
    echo ""
    echo "Android device detected. Do you want to install the app now? (y/n)"
    read -r answer
    if [[ "$answer" == "y" ]]; then
      echo "Installing app..."
      adb install -r "gen/android/app/build/outputs/apk/debug/app-debug.apk"
      echo "App installed!"
      
      # Option to launch the app
      echo "Do you want to launch the app? (y/n)"
      read -r launch
      if [[ "$launch" == "y" ]]; then
        echo "Launching app..."
        adb shell monkey -p net.blipty.app -c android.intent.category.LAUNCHER 1
      fi
    fi
  else
    echo "No Android device detected. Connect a device or start an emulator to install the app."
    echo "You can start the Android TV emulator with:"
    echo "./setup-tv-emulator.sh"
  fi
else
  echo "❌ Build failed!"
  echo "Check the logs above for any errors."
fi
