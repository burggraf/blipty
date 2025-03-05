#!/bin/bash

# Make script exit on any error
set -e

echo "Setting up Android SDK and NDK environment variables..."

# Check if Android Studio is installed in the default location
ANDROID_SDK_PATH="$HOME/Library/Android/sdk"

if [ ! -d "$ANDROID_SDK_PATH" ]; then
    echo "Android SDK not found in $ANDROID_SDK_PATH"
    echo "Please install Android Studio and SDK first from https://developer.android.com/studio"
    exit 1
fi

# Find NDK within SDK directory
if [ -d "$ANDROID_SDK_PATH/ndk" ]; then
    # Get the latest NDK version
    NDK_VERSION=$(ls -1 "$ANDROID_SDK_PATH/ndk" | sort -V | tail -n 1)
    if [ -n "$NDK_VERSION" ]; then
        NDK_PATH="$ANDROID_SDK_PATH/ndk/$NDK_VERSION"
        echo "Found NDK version $NDK_VERSION"
    else
        echo "No NDK versions found in $ANDROID_SDK_PATH/ndk"
        echo "Please install NDK using Android Studio's SDK Manager"
        exit 1
    fi
else
    echo "NDK directory not found in $ANDROID_SDK_PATH"
    echo "Please install NDK using Android Studio's SDK Manager"
    exit 1
fi

# Export environment variables
export ANDROID_HOME="$ANDROID_SDK_PATH"
export NDK_HOME="$NDK_PATH"
export PATH="$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools"

echo "Android environment variables set:"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "NDK_HOME=$NDK_HOME"
echo "PATH updated to include Android tools"

# Make the variables persist in the current shell
cat << EOF > .env
export ANDROID_HOME="$ANDROID_SDK_PATH"
export NDK_HOME="$NDK_PATH"
export PATH="\$PATH:\$ANDROID_HOME/tools:\$ANDROID_HOME/platform-tools"
EOF

echo ""
echo "Environment variables have been set and saved to .env"
echo "To make these variables persistent in new terminals, add this line to your ~/.zshrc or ~/.bashrc:"
echo "source $(pwd)/.env"
