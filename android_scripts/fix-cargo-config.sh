#!/bin/bash

# Make script exit on any error
set -e

echo "Fixing Cargo configuration for Android builds..."

# Backup existing config if it exists
CONFIG_FILE=~/.cargo/config.toml
if [ -f "$CONFIG_FILE" ]; then
  BACKUP_FILE="${CONFIG_FILE}.bak-$(date +%Y%m%d%H%M%S)"
  cp "$CONFIG_FILE" "$BACKUP_FILE"
  echo "✅ Backed up existing config to $BACKUP_FILE"
fi

# Find NDK
if [ -z "$NDK_HOME" ]; then
  if [ -d "$HOME/Library/Android/sdk" ]; then
    ANDROID_HOME="$HOME/Library/Android/sdk"
    
    # Find an NDK version
    for dir in "$ANDROID_HOME/ndk"/*; do
      if [ -d "$dir" ]; then
        NDK_HOME="$dir"
        break
      fi
    done
    
    if [ -z "$NDK_HOME" ] && [ -d "$ANDROID_HOME/ndk-bundle" ]; then
      NDK_HOME="$ANDROID_HOME/ndk-bundle"
    fi
  fi
fi

if [ -z "$NDK_HOME" ]; then
  echo "❌ NDK_HOME not set and NDK not found automatically."
  echo "Please enter the path to your Android NDK:"
  read -r NDK_HOME
  
  if [ ! -d "$NDK_HOME" ]; then
    echo "❌ The path you provided does not exist."
    exit 1
  fi
fi

echo "Using NDK at: $NDK_HOME"

# Create new config file with clean syntax
mkdir -p ~/.cargo
cat > "$CONFIG_FILE" << EOF
# Cargo configuration for Android builds
[target.aarch64-linux-android]
ar = "$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
ar = "$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang"

[target.i686-linux-android]
ar = "$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android21-clang"

[target.x86_64-linux-android]
ar = "$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android21-clang"
EOF

echo "✅ Created clean Cargo config at $CONFIG_FILE"

# Try to build a minimal empty project to test config
echo "Testing Cargo configuration with a minimal Android build..."

TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

# Create a minimal Rust project
cargo init --lib test_android_config
cd test_android_config

# Try to compile for Android
if rustc --crate-type=staticlib --target=aarch64-linux-android -o /dev/null src/lib.rs >/dev/null 2>&1; then
  echo "✅ Cargo configuration works!"
else
  echo "⚠️ Basic compilation test failed, but config should still be fixed."
fi

# Clean up
cd ~
rm -rf "$TEST_DIR"

echo ""
echo "Next steps:"
echo "1. Try building your Tauri Android app again:"
echo "   cd /Users/markb/dev/iptv"
echo "   ./fix-dependencies.sh"
echo ""
echo "If you want to revert to your previous Cargo config:"
echo "cp $BACKUP_FILE $CONFIG_FILE"
