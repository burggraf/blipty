#!/bin/bash

# Make script exit on any error
set -e

echo "Setting up Android TV emulator..."

# Make sure sdkmanager is available
if [ ! -f "$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager" ] && [ ! -f "$ANDROID_HOME/tools/bin/sdkmanager" ]; then
  echo "❌ Command-line tools not found. Please install them via Android Studio SDK Manager."
  echo "1. Open Android Studio"
  echo "2. Go to Tools > SDK Manager"
  echo "3. Select SDK Tools tab"
  echo "4. Check 'Android SDK Command-line Tools'"
  echo "5. Click Apply and wait for installation to complete"
  exit 1
fi

# Determine sdkmanager path
if [ -f "$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager" ]; then
  SDKMANAGER="$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager"
else
  SDKMANAGER="$ANDROID_HOME/tools/bin/sdkmanager"
fi

echo "Using sdkmanager at: $SDKMANAGER"

# Install Android TV system image if not already installed
TV_IMAGE="system-images;android-30;google_atv;x86"
echo "Installing Android TV system image ($TV_IMAGE)..."
echo "This may take several minutes..."
echo "y" | $SDKMANAGER "$TV_IMAGE"

# Create TV AVD (Android Virtual Device)
echo "Creating Android TV emulator..."
echo "no" | avdmanager create avd -n "Android_TV" -k "$TV_IMAGE" --device "tv_1080p" --force

echo "✅ Android TV emulator setup complete!"
echo ""
echo "To start the emulator, run:"
echo "emulator -avd Android_TV"
echo ""
echo "Once the emulator is running, you can install your app using:"
echo "adb install src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk"
