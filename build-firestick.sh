#!/bin/bash

# Build optimized APK for FireStick
echo "Building optimized APK for FireStick..."

# Build the frontend
bun run build

# Build the Android app with firestick configuration
cargo tauri android build --target aarch64 --debug

# The APK will be in src-tauri/gen/android/app/build/outputs/apk/firestick/debug/
echo "Build complete! Look for the APK in src-tauri/gen/android/app/build/outputs/apk/firestick/debug/"