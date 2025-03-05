#!/bin/bash

# Make script exit on any error
set -e

echo "Finding and setting Android NDK environment variable..."

# Find Android SDK
POSSIBLE_SDK_LOCATIONS=(
  "$HOME/Library/Android/sdk"
  "/Applications/Android Studio.app/Contents/sdk"
  "$HOME/Android/Sdk"
)

for location in "${POSSIBLE_SDK_LOCATIONS[@]}"; do
  if [ -d "$location" ]; then
    SDK_LOCATION="$location"
    break
  fi
done

if [ -z "$SDK_LOCATION" ]; then
  echo "❌ Android SDK not found! Please install Android Studio."
  exit 1
fi

echo "✅ Found Android SDK at: $SDK_LOCATION"

# Export ANDROID_HOME for this session
export ANDROID_HOME="$SDK_LOCATION"

# Find NDK within SDK directory - first try ndk directory with subdirectories
NDK_FOUND=false

if [ -d "$ANDROID_HOME/ndk" ]; then
  echo "Checking $ANDROID_HOME/ndk for NDK versions..."
  
  # List all subdirectories and sort by version (newest first)
  NDK_DIRS=()
  for dir in "$ANDROID_HOME/ndk"/*; do
    if [ -d "$dir" ]; then
      NDK_DIRS+=("$dir")
    fi
  done
  
  # Sort NDK directories by version (assuming version number in directory name)
  if [ ${#NDK_DIRS[@]} -gt 0 ]; then
    # Use the first (latest) NDK found
    export NDK_HOME="${NDK_DIRS[0]}"
    echo "✅ Found Android NDK at: $NDK_HOME"
    NDK_FOUND=true
  fi
fi

# Try ndk-bundle if not found in ndk directory
if [ "$NDK_FOUND" = false ] && [ -d "$ANDROID_HOME/ndk-bundle" ]; then
  export NDK_HOME="$ANDROID_HOME/ndk-bundle"
  echo "✅ Found Android NDK at: $NDK_HOME"
  NDK_FOUND=true
fi

# Check if we found NDK
if [ "$NDK_FOUND" = false ]; then
  echo "❌ Android NDK not found! Please install NDK through Android Studio SDK Manager."
  echo "1. Open Android Studio"
  echo "2. Go to Tools > SDK Manager"
  echo "3. Select the SDK Tools tab"
  echo "4. Check 'NDK (Side by side)' or 'NDK' and click Apply/OK"
  exit 1
fi

# Show the NDK path for clarity
echo "Using NDK_HOME=$NDK_HOME"

# Create/update .env file for future use
echo "export ANDROID_HOME=\"$ANDROID_HOME\"" > .env
echo "export NDK_HOME=\"$NDK_HOME\"" >> .env
echo "export PATH=\"\$PATH:\$ANDROID_HOME/tools:\$ANDROID_HOME/tools/bin:\$ANDROID_HOME/platform-tools\"" >> .env

echo "Saved environment variables to .env file"
echo "Use 'source .env' to load them in other terminal sessions"

echo ""
echo "🚀 Environment variables set for current session!"
echo "Would you like to run the android build now? (y/n)"
read -r answer

if [[ "$answer" == "y" ]]; then
  echo "Running Android build..."
  cd src-tauri
  ANDROID_HOME="$ANDROID_HOME" NDK_HOME="$NDK_HOME" cargo tauri android build --debug
  
  if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
  else
    echo "❌ Build failed. Check errors above."
  fi
else
  echo "To build later, run: source .env && cd src-tauri && cargo tauri android build --debug"
fi
