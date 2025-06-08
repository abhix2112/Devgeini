#!/bin/bash

set -e

BINARY_NAME="devgeini"
INSTALL_DIR="/usr/local/bin"
DOWNLOAD_URL="https://github.com/abhix2112/Devgeini/releases/latest/download/devgeini"  # <-- Make sure this is correct

echo "📥 Downloading $BINARY_NAME from GitHub..."
curl -L "$DOWNLOAD_URL" -o "$BINARY_NAME"
chmod +x "$BINARY_NAME"

echo "📦 Installing to $INSTALL_DIR (you may need sudo)..."
sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

if command -v $BINARY_NAME >/dev/null 2>&1; then
    echo "✅ '$BINARY_NAME' is now installed globally!"
    echo "➡️  You can run it using: $BINARY_NAME"
else
    echo "❌ Installation failed or $INSTALL_DIR is not in your PATH."
    echo "   Try adding it manually or run: export PATH=\"$INSTALL_DIR:\$PATH\""
fi
