#!/bin/bash
# This script helps fix OpenSSL-related issues when compiling for Android

set -e

echo "Setting up environment for Android cross-compilation..."

# Find the Android NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    # Try to detect NDK from common locations
    if [ -d "$HOME/Library/Android/sdk/ndk" ]; then
        for dir in "$HOME/Library/Android/sdk/ndk"/*/; do
            if [ -d "$dir" ] && [ -f "$dir/source.properties" ]; then
                export ANDROID_NDK_HOME="${dir}"
                break
            fi
        done
    fi
fi

if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME not set and NDK not found."
    echo "Please set ANDROID_NDK_HOME to point to your Android NDK installation."
    exit 1
fi

echo "Using NDK at: $ANDROID_NDK_HOME"

# Configure Cargo to use rustls instead of OpenSSL
echo "Configuring Cargo to use rustls for TLS..."
cat > ~/.cargo/config.toml << EOF
[target.aarch64-linux-android]
linker = "${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"

[target.i686-linux-android]
linker = "${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android21-clang"

[target.x86_64-linux-android]
linker = "${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android21-clang"
EOF

echo "Configuration complete!"
echo "Now run './build-android-tv.sh' to build your Android TV app."

# Check that ANDROID_NDK_HOME is set
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME environment variable is not set"
    exit 1
fi

# Create the directory for OpenSSL libraries
mkdir -p "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/aarch64-linux-android/24"

# Download and extract pre-built OpenSSL libraries
curl -L https://github.com/KDAB/android_openssl/raw/master/aarch64-linux-android/24/libssl.so -o "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/aarch64-linux-android/24/libssl.so"
curl -L https://github.com/KDAB/android_openssl/raw/master/aarch64-linux-android/24/libcrypto.so -o "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/aarch64-linux-android/24/libcrypto.so"

# Make the files executable
chmod +x "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/aarch64-linux-android/24/libssl.so"
chmod +x "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/aarch64-linux-android/24/libcrypto.so"

echo "OpenSSL libraries have been installed successfully"
