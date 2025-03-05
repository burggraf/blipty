#!/bin/bash

# Make script exit on any error
set -e

echo "Fixing reqwest import in fetch_api.rs..."

FILE_PATH="src/channel_commands/fetch_api.rs"
if [ ! -f "$FILE_PATH" ]; then
  # Try src-tauri path
  FILE_PATH="src-tauri/src/channel_commands/fetch_api.rs"
  if [ ! -f "$FILE_PATH" ]; then
    echo "❌ Could not find fetch_api.rs file"
    exit 1
  fi
fi

# Create backup
cp "$FILE_PATH" "${FILE_PATH}.bak"
echo "Created backup at ${FILE_PATH}.bak"

# Check the file contents for duplicate imports
if grep -q "use reqwest::{Client, ClientBuilder}" "$FILE_PATH"; then
  echo "Found duplicate imports, fixing file..."
  
  # Create a new file with fixed imports
  TMP_FILE=$(mktemp)
  
  # Write the fixed content
  cat > "$TMP_FILE" << EOF
// For Android, we use a minimal reqwest client without native TLS
#[cfg(target_os = "android")]
use reqwest::{Client, ClientBuilder};

// For other platforms, use the default reqwest
#[cfg(not(target_os = "android"))]
use reqwest;

// Rest of your file continues here
EOF

  # Append the rest of the file, skipping the problematic imports
  grep -v "use reqwest" "$FILE_PATH" | tail -n +2 >> "$TMP_FILE"
  
  # Replace the original file
  mv "$TMP_FILE" "$FILE_PATH"
  
  echo "✅ Fixed imports in $FILE_PATH"
else
  echo "⚠️ Could not find duplicate imports pattern"
fi

echo ""
echo "Now try building again with:"
echo "cd src-tauri"
echo "ANDROID_HOME=\"$HOME/Library/Android/sdk\" NDK_HOME=\"\$ANDROID_HOME/ndk/25.2.9519653\" cargo tauri android build --debug"
echo ""
echo "Or using the minimal-build.sh script:"
echo "./minimal-build.sh"
