#!/bin/bash

echo "Checking Android SDK and NDK paths..."

# Check if Android Studio is installed in common locations
POSSIBLE_SDK_LOCATIONS=(
  "$HOME/Library/Android/sdk"
  "/Applications/Android Studio.app/Contents/sdk"
  "$HOME/Android/Sdk"
)

SDK_FOUND=false

for location in "${POSSIBLE_SDK_LOCATIONS[@]}"; do
  if [ -d "$location" ]; then
    echo "✅ Found Android SDK at: $location"
    ANDROID_HOME="$location"
    SDK_FOUND=true
    break
  fi
done

if [ "$SDK_FOUND" = false ]; then
  echo "❌ Could not find Android SDK in common locations."
  exit 1
fi

echo ""
echo "Checking for NDK installation..."

# Check for NDK in potential locations
echo "Looking in $ANDROID_HOME/ndk..."
if [ -d "$ANDROID_HOME/ndk" ]; then
  echo "  Found NDK directory. Checking for versions:"
  ls -la "$ANDROID_HOME/ndk"
  
  # Find the latest NDK version
  LATEST_NDK=""
  for ndk_dir in "$ANDROID_HOME/ndk"/*; do
    if [ -d "$ndk_dir" ]; then
      echo "  Potential NDK: $ndk_dir"
      LATEST_NDK="$ndk_dir"
    fi
  done
  
  if [ -n "$LATEST_NDK" ]; then
    echo "  Latest NDK seems to be: $LATEST_NDK"
    echo ""
    echo "Checking contents of this NDK directory:"
    ls -la "$LATEST_NDK"
  else
    echo "  No NDK versions found in $ANDROID_HOME/ndk"
  fi
else
  echo "  NDK directory not found."
fi

echo ""
echo "Checking for ndk-bundle..."
if [ -d "$ANDROID_HOME/ndk-bundle" ]; then
  echo "  Found ndk-bundle at $ANDROID_HOME/ndk-bundle"
  ls -la "$ANDROID_HOME/ndk-bundle"
else
  echo "  ndk-bundle not found."
fi

echo ""
echo "=== Setting environment variables ==="
echo "Try adding the following to your shell profile:"
echo ""

if [ -d "$ANDROID_HOME/ndk" ]; then
  # Get the newest NDK version
  NEWEST_NDK=$(find "$ANDROID_HOME/ndk" -maxdepth 1 -type d -name "[0-9]*" | sort -r | head -n 1)
  if [ -n "$NEWEST_NDK" ]; then
    echo "export ANDROID_HOME=\"$ANDROID_HOME\""
    echo "export NDK_HOME=\"$NEWEST_NDK\""
    echo "export PATH=\"\$PATH:\$ANDROID_HOME/tools:\$ANDROID_HOME/tools/bin:\$ANDROID_HOME/platform-tools\""
    
    echo ""
    echo "Or for temporary use in current terminal:"
    echo ""
    echo "ANDROID_HOME=\"$ANDROID_HOME\" NDK_HOME=\"$NEWEST_NDK\" cargo tauri android build --debug"
  else
    echo "❌ No NDK version directories found in $ANDROID_HOME/ndk"
    echo "Please install NDK using Android Studio SDK Manager"
  fi
elif [ -d "$ANDROID_HOME/ndk-bundle" ]; then
  echo "export ANDROID_HOME=\"$ANDROID_HOME\""
  echo "export NDK_HOME=\"$ANDROID_HOME/ndk-bundle\""
  echo "export PATH=\"\$PATH:\$ANDROID_HOME/tools:\$ANDROID_HOME/tools/bin:\$ANDROID_HOME/platform-tools\""
  
  echo ""
  echo "Or for temporary use in current terminal:"
  echo ""
  echo "ANDROID_HOME=\"$ANDROID_HOME\" NDK_HOME=\"$ANDROID_HOME/ndk-bundle\" cargo tauri android build --debug"
else
  echo "❌ No NDK installation found"
  echo "Please install NDK using Android Studio SDK Manager"
fi
