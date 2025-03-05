#!/bin/bash

# EDIT THESE PATHS TO MATCH YOUR SYSTEM
export ANDROID_HOME="$HOME/Library/Android/sdk"
export NDK_HOME="$HOME/Library/Android/sdk/ndk/25.2.9519653"  # <- REPLACE WITH YOUR NDK PATH

echo "Using:"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "NDK_HOME=$NDK_HOME"

# Clean up previous build if needed
rm -rf src-tauri/gen/android

# Initialize Android project 
cd src-tauri
cargo tauri android init

# Apply Android TV configuration to the manifest
if [ -f "gen/android/app/src/main/AndroidManifest.xml" ]; then
  sed -i.bak '/<\/activity>/i \
            <intent-filter> \
                <action android:name="android.intent.action.MAIN" /> \
                <category android:name="android.intent.category.LEANBACK_LAUNCHER" /> \
            </intent-filter>' \
    gen/android/app/src/main/AndroidManifest.xml
  
  sed -i.bak '/<uses-permission/i \
    <!-- Android TV support --> \
    <uses-feature android:name="android.software.leanback" android:required="false" /> \
    <uses-feature android:name="android.hardware.touchscreen" android:required="false" />' \
    gen/android/app/src/main/AndroidManifest.xml
    
  echo "Updated AndroidManifest.xml for Android TV support"
fi

# Build the app
echo "Building app..."
cargo tauri android build --debug

# Check if build was successful
if [ -f "gen/android/app/build/outputs/apk/debug/app-debug.apk" ]; then
  echo "✅ Build completed successfully!"
  echo "APK: $(pwd)/gen/android/app/build/outputs/apk/debug/app-debug.apk"
  
  # Offer to install to a connected device
  if adb devices | grep -q "device$"; then
    echo ""
    echo "Android device detected. Install now? (y/n)"
    read -r answer
    if [ "$answer" = "y" ]; then
      adb install -r "gen/android/app/build/outputs/apk/debug/app-debug.apk"
    fi
  fi
else
  echo "❌ Build failed"
fi
