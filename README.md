# Shred Extension Rs

[![Build Status](https://img.shields.io/github/actions/workflow/status/williamcanin/shred-extension-rs/release.yml?logo=github)](https://github.com/williamcanin/shred-extension-rs/actions)
[![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20GNOME%20%26%20XFCE-blue?logo=linux)](#)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

A native, ultra-lightweight extension for the **Nautilus (GNOME)** and **Thunar (XFCE)** file managers that adds a permanent, government‑grade secure deletion option (**Shred**) to your file context menu.

Written in **Rust**, the extension focuses on high performance and maximum data destruction security. It communicates directly with low-level C APIs (GObject / libnautilus-extension / libthunarx) without bloated dependency layers.

## ✨ Features

* **Native Context Menu Integration**: A clear "Secure Delete" option available for any file in GNOME or XFCE file managers.
* **Secure Empty Trash**: Right-click the Trash icon or its background to securely wipe all deleted files. It recursively shreds the actual data on disk before removing entries from the trash system.
* **Integrated Confirmation Dialog**: To prevent accidental clicks and the headaches the `shred` command can cause when misused, the extension calls the system’s native dialog (`zenity`). A "Yes/OK" confirmation is required.
* **Smart File Camouflage**: Unlike the native `shred -u` behavior in the terminal (which briefly pollutes your folder with renamed artifacts like `000000`), this extension hides files inside temporary "invisible" directories (starting with `.`). The deletion happens behind the scenes, silently.
* **Modular Architecture (Workspace)**: Now structured as a Rust Workspace with zero cross-manager warnings. You can build specific extensions for Nautilus or Thunar without bloating the binary.
* **Built‑in Internationalization (i18n)**: Automatic language detection based on the system locale to display menus and messages in:

  * 🇧🇷 Portuguese‑BR / PT ("Excluir com Segurança" / "Esvaziar Lixeira com Segurança")
  * 🇪🇸 Spanish ("Eliminación Segura" / "Vaciar Papelera de Forma Segura")
  * 🇺🇸 English / Fallback ("Secure Delete" / "Secure Empty Trash")

---

## 📦 Installation & Build

You can install the extension in three different ways:

1. **Arch Linux (AUR)** — separate packages for each file manager
2. **Universal automatic script (any distro)**
3. **Manual compilation (from source)**

---

### 🐧 Arch Linux (AUR)

If you use Arch Linux or derivatives (EndeavourOS, Manjaro, CachyOS, etc.), install directly from the AUR:

*   **Nautilus**: `yay -S shred-extension-rs-nautilus`
*   **Thunar**: `yay -S shred-extension-rs-thunar`

Both packages install the library in the correct system path.

---

### 🚀 Automatic Installation (Any Linux Distribution)

You can install **without downloading anything** by running the script directly from the official repository:

```bash
bash <(curl -fsSL https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh)
```

---

### 🛠 Manual Compilation

To build from source, ensure you have `rustup` and `pkg-config` installed.

1. **Clone the repository**:
   ```bash
   git clone https://github.com/williamcanin/shred-extension-rs.git
   cd shred-extension-rs
   ```

2. **Build for your file manager**:
   * **Nautilus**:
     ```bash
     cargo build -p shred-nautilus --release
     sudo cp target/release/libshred_nautilus.so /usr/lib/nautilus/extensions-4/libshred_extension_rs.so
     nautilus -q
     ```
   * **Thunar**:
     ```bash
     cargo build -p shred-thunar --release
     sudo cp target/release/libshred_thunar.so /usr/lib/thunarx-3/libshred_extension_rs.so
     thunar -q
     ```

> [!TIP]
> Each extension depends on the `shred-common` internal crate, ensuring that shared logic (like the shredding algorithm) remains consistent across all platforms while keeping the final binaries lightweight.

---

## 🧑‍💻 Motivation & Backend Architecture

If you are interested in Software Engineering, C‑FFI integration, and how we worked around Rust library limitations that block the GTK *main thread*, check out [ARCHITECTURE.md](ARCHITECTURE.md) for implementation details.

---

**Usage Warning:** *The overwrite process runs in 3 stages followed by zero‑filling. This tool performs high‑security deletion and the process is **irreversible**. Make sure you know exactly what you are clicking.*
