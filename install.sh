#!/usr/bin/env bash
# Author: William C. Canin <hello.williamcanin@gmail.com>
set -e

# ----- Seleciona o file manager alvo -----

echo "Which file manager do you want to install the extension for?"
echo "  1) Nautilus (GNOME)"
echo "  2) Thunar (XFCE)"
read -rp "Choice [1/2]: " FM_CHOICE

case "$FM_CHOICE" in
    1)
        FM="nautilus"
        LIB_PREFIX="libshred-extension-rs-nautilus"
        REPO="williamcanin/shred-extension-rs"
        ;;
    2)
        FM="thunar"
        LIB_PREFIX="libshred-extension-rs-thunar"
        REPO="williamcanin/shred-extension-rs"
        ;;
    *)
        echo "Error: Invalid choice."
        exit 1
        ;;
esac

# ----- Localiza ou baixa a biblioteca -----

LOCAL_LIB=$(ls ${LIB_PREFIX}*.so 2>/dev/null | head -n 1)

if [ -n "$LOCAL_LIB" ]; then
    LIB_NAME="$LOCAL_LIB"
    echo "Found local library: $LIB_NAME"
else
    echo "Library not found locally."
    echo "Attempting to download the latest release from GitHub..."

    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        echo "Error: Neither 'curl' nor 'wget' was found."
        echo "Please install curl/wget or download the file manually."
        exit 1
    fi

    echo "Fetching latest version tag from GitHub API..."
    API_URL="https://api.github.com/repos/${REPO}/releases/latest"

    if command -v curl >/dev/null 2>&1; then
        VERSION_TAG=$(curl -s "$API_URL" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/')
    else
        VERSION_TAG=$(wget -qO- "$API_URL" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/')
    fi

    if [ -z "$VERSION_TAG" ]; then
        echo "Error: Could not retrieve the latest release version from GitHub."
        exit 1
    fi

    LIB_NAME="${LIB_PREFIX}-${VERSION_TAG}-x86_64.so"
    DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${LIB_NAME}"
    echo "Target file: $LIB_NAME"

    if command -v curl >/dev/null 2>&1; then
        if curl -L --fail --progress-bar "$DOWNLOAD_URL" -o "$LIB_NAME"; then
            echo "Download completed successfully."
        else
            echo "Error: Failed to download the latest release."
            rm -f "$LIB_NAME"
            exit 1
        fi
    else
        if wget -q --show-progress "$DOWNLOAD_URL" -O "$LIB_NAME"; then
            echo "Download completed successfully."
        else
            echo "Error: Failed to download the latest release."
            rm -f "$LIB_NAME"
            exit 1
        fi
    fi
fi

# ----- Detecta o diretório de extensões -----

EXT_DIR=""

if [ "$FM" = "nautilus" ]; then
    if command -v pkg-config >/dev/null 2>&1; then
        EXT_DIR=$(pkg-config --variable=extensiondir libnautilus-extension 2>/dev/null || true)
    fi
    if [ -z "$EXT_DIR" ]; then
        if   [ -d "/usr/lib/x86_64-linux-gnu/nautilus/extensions-4" ]; then
            EXT_DIR="/usr/lib/x86_64-linux-gnu/nautilus/extensions-4"
        elif [ -d "/usr/lib64/nautilus/extensions-4" ]; then
            EXT_DIR="/usr/lib64/nautilus/extensions-4"
        else
            EXT_DIR="/usr/lib/nautilus/extensions-4"
        fi
    fi
else
    if command -v pkg-config >/dev/null 2>&1; then
        EXT_DIR=$(pkg-config --variable=extensionsdir thunarx-3 2>/dev/null || true)
    fi
    if [ -z "$EXT_DIR" ]; then
        if   [ -d "/usr/lib/x86_64-linux-gnu/thunarx-3" ]; then
            EXT_DIR="/usr/lib/x86_64-linux-gnu/thunarx-3"
        elif [ -d "/usr/lib64/thunarx-3" ]; then
            EXT_DIR="/usr/lib64/thunarx-3"
        else
            EXT_DIR="/usr/lib/thunarx-3"
        fi
    fi
fi

echo "Extension directory detected: $EXT_DIR"
echo "You will be asked for your administrator privileges (sudo) to install the lib."

# ----- Instala -----

sudo mkdir -p "$EXT_DIR"
# Remove versões antigas para evitar conflito
sudo rm -f "$EXT_DIR"/${LIB_PREFIX}*.so
sudo cp -f "$LIB_NAME" "$EXT_DIR/"
sudo chmod 755 "$EXT_DIR/$LIB_NAME"

echo ""
echo "Installation completed successfully!"

# ----- Reinicia o file manager -----

if [ "$FM" = "nautilus" ]; then
    echo "Restarting Nautilus..."
    nautilus -q 2>/dev/null || true
    echo "Ready! Open Nautilus, right-click on a file, and test!"
else
    echo "Restarting Thunar..."
    thunar -q 2>/dev/null || true
    echo "Ready! Open Thunar, right-click on a file, and test!"
fi
