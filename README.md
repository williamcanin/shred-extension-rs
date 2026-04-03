# Shred Extension Rs

[![Build Status](https://img.shields.io/github/actions/workflow/status/williamcanin/shred-extension-rs/release.yml?logo=github)](https://github.com/williamcanin/shred-extension-rs/actions)
[![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20GNOME%20%26%20XFCE-blue?logo=linux)](#)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

A native, ultra-lightweight extension for the **Nautilus (GNOME)** and **Thunar (XFCE)** file managers that adds a permanent, government‑grade secure deletion option (**Shred**) to your file context menu.

Written in **Rust**, the extension focuses on high performance and maximum data destruction security. It communicates directly with low-level C APIs (GObject / libnautilus-extension / libthunarx) without bloated dependency layers.

## ✨ Features

* **Native Context Menu Integration**: A clear "Secure Delete" option available for any file in GNOME or XFCE file managers.
* **Asynchronous & Instant UI**: After confirmation, the file disappears instantly from the interface. The actual shredding runs silently in a **background thread**, keeping the file manager fully responsive.
* **Integrated Confirmation Dialog**: To prevent accidental clicks and the headaches the `shred` command can cause when misused, the extension calls the system’s native dialog (`zenity`). A "Yes/OK" confirmation is required.
* **Smart File Camouflage**: Unlike the native `shred -u` behavior in the terminal (which briefly pollutes your folder with renamed artifacts like `000000`), this extension hides files inside temporary "invisible" directories (starting with `.`). The deletion happens behind the scenes, silently.
* **Multi‑Environment Support**: Works with both **Nautilus** and **Thunar**, sharing the same Rust codebase.
* **Built‑in Internationalization (i18n)**: Automatic language detection based on the system locale to display menus and messages in:

  * 🇧🇷 Portuguese‑BR / PT ("Excluir com Segurança")
  * 🇪🇸 Spanish ("Eliminación Segura")
  * 🇺🇸 English / Fallback ("Secure Delete")

---

## 📦 Installation

You can install the extension in three different ways:

1. **Arch Linux (AUR)** — separate packages for each file manager
2. **Universal automatic script (any distro)**
3. **Manual library installation**

---

### 🐧 Arch Linux (AUR)

If you use Arch Linux or derivatives (EndeavourOS, Manjaro, CachyOS, etc.), install directly from the AUR by choosing your desired file manager:

```bash
yay -S shred-extension-rs-nautilus
```

or

```bash
yay -S shred-extension-rs-thunar
```

Or build manually:

```bash
git clone https://aur.archlinux.org/shred-extension-rs-nautilus.git
cd shred-extension-rs-nautilus
makepkg -si
```

or

```bash
git clone https://aur.archlinux.org/shred-extension-rs-thunar.git
cd shred-extension-rs-thunar
makepkg -si
```

Both packages install the library in the correct system path using the standard name:

```
libshred_extension_rs.so
```

Fully compatible with future updates and with no conflict with the script installer.

---

### 🚀 Automatic Installation (Any Linux Distribution)

You can install **without downloading anything** by running the script directly from the official repository:

```bash
bash <(curl -fsSL https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh)
```

Or using `wget`:

```bash
bash <(wget -qO- https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh)
```

The script will:

* Ask which file manager you use (Nautilus or Thunar)
* Automatically download the latest release via the GitHub API
* Install the library in the correct system directory
* Standardize the name to `libshred_extension_rs.so`, ensuring AUR compatibility

> 💡 Security tip: you can inspect the script before running it by opening:
> [https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh](https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh)

---

### 🧰 Manual Installation

If you prefer managing everything manually with `root` commands, download the `.so` library from the project Releases.

After downloading, rename it to the expected standard:

```bash
mv libshred-extension-rs-*.so libshred_extension_rs.so
```

* **For Nautilus:**

```bash
sudo cp libshred_extension_rs.so /usr/lib/nautilus/extensions-4/
nautilus -q
```

*(Some systems like Ubuntu/Mint may use `/usr/lib/x86_64-linux-gnu/nautilus/extensions-4/`)*

* **For Thunar:**

```bash
sudo cp libshred_extension_rs.so /usr/lib/thunarx-3/
thunar -q
```

*(Some distributions use `/usr/lib/x86_64-linux-gnu/thunarx-3/`)*

---

## 🧑‍💻 Motivation & Backend Architecture

If you are interested in Software Engineering, C‑FFI integration, and how we worked around Rust library limitations that block the GTK *main thread*, check out [ARCHITECTURE.md](ARCHITECTURE.md) for implementation details.

---

**Usage Warning:** *The overwrite process runs in 3 stages followed by zero‑filling. This tool performs high‑security deletion and the process is **irreversible**. Make sure you know exactly what you are clicking.*
