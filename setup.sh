=#!/bin/bash
set -e

BINARY_NAME="devgeini"
INSTALL_DIR="/usr/local/bin"
BASE_URL="https://github.com/abhix2112/Devgeini/releases/latest/download"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names to your naming convention
case $ARCH in
    x86_64|amd64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="arm64"  # Add if you have ARM builds
        ;;
    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Determine the correct filename based on OS
case $OS in
    linux)
        FILENAME="devgeini-linux-${ARCH}"
        ;;
    darwin)
        FILENAME="devgeini-macos-${ARCH}"
        ;;
    mingw*|msys*|cygwin*)
        FILENAME="devgeini-windows-${ARCH}.exe"
        BINARY_NAME="devgeini.exe"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

DOWNLOAD_URL="${BASE_URL}/${FILENAME}"

echo "🔍 Detected: $OS $ARCH"
echo "📥 Downloading $FILENAME from GitHub..."

# Download with error handling
if ! curl -L "$DOWNLOAD_URL" -o "$BINARY_NAME"; then
    echo "❌ Failed to download $BINARY_NAME from $DOWNLOAD_URL"
    echo "   Please check if the release exists and try again."
    exit 1
fi

# Verify file was downloaded and isn't empty
if [ ! -s "$BINARY_NAME" ]; then
    echo "❌ Downloaded file is empty or corrupted"
    rm -f "$BINARY_NAME"
    exit 1
fi

chmod +x "$BINARY_NAME"

echo "📦 Installing to $INSTALL_DIR (you may need sudo)..."

# Install with error handling
if ! sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"; then
    echo "❌ Failed to install to $INSTALL_DIR"
    echo "   Make sure you have sudo permissions or try a different install directory"
    exit 1
fi

# Verify installation
if command -v ${BINARY_NAME%.*} >/dev/null 2>&1; then
    echo "✅ '$BINARY_NAME' is now installed globally!"
    echo "➡️  You can run it using: ${BINARY_NAME%.*}"
else
    echo "❌ Installation failed or $INSTALL_DIR is not in your PATH."
    echo "   Try adding it manually or run: export PATH=\"$INSTALL_DIR:\$PATH\""
    # Clean up on failure
    sudo rm -f "$INSTALL_DIR/$BINARY_NAME" 2>/dev/null || true
    exit 1
fi
