# 🏗 Shred Extension Rs Architecture (C‑FFI & Rust) 🦀

Building an extension for the GNOME ecosystem (Nautilus using GTK4/Libadwaita APIs) and XFCE (Thunar) is not straightforward given the evolution and deprecation of older Rust crates. This document explains how the C‑FFI barriers were overcome and the design decisions that allow this project to support both file managers simultaneously from a single unified Rust codebase.

---

## 1. Abandoning Outdated Rust Crates and Embracing “Pure FFI”

Early attempts relied on community crates such as `thunar-extension` or older Nautilus bindings. These crates were frequently tied to outdated C dependencies (for example, `gtk-sys v0.15` bound to GTK3).

With GTK4 (GNOME 43+) and recent Thunar updates, several legacy C structures were removed or changed from the extension APIs. Simply linking against these old crates resulted in a cascade of **"Undefined Symbols"** errors when the extension was loaded by the file manager.

### The Multi‑Environment Solution

All obsolete binding crates were discarded. Instead, this project implements its **own C‑FFI bindings (Rust FFI)** that point directly to the native memory and symbols provided by each file manager (`libnautilus-extension` and `libthunarx`).

Using conditional compilation such as:

```rust
#[cfg(feature = "nautilus")]
#[cfg(feature = "thunar")]
```

Rust exposes the correct VTable and function pointers for each environment at compile time.

Rather than depending on the entire Rust GTK ecosystem, the extension uses only minimal and safe conversions via native `gio`, and registers the menu provider directly with `g_type_module_register_type`.

This results in a lightweight, future‑proof, and highly compatible integration layer.

---

## 2. Nautilus / Thunar UI Freezing (Main Thread Blocking)

During early implementations of the shredding logic, performing multi‑megabyte overwrite loops directly inside the context‑menu callback caused the entire file manager UI to **freeze**. The GTK/GLib main thread would block until the deletion process finished.

### The Asynchronous Solution

A simple but effective concurrency model was implemented using:

```rust
std::thread::spawn
```

The heavy shredding process runs in **separate background threads**, allowing the original GTK callback to return immediately. From the user’s perspective, the file disappears almost instantly while the actual disk overwrite continues silently in the background.

This preserves responsiveness and provides a seamless user experience.

---

## 3. The Native Visual Problem of `shred -u`

Using the traditional `-u` flag forces the native Linux `shred` binary to repeatedly rename the file with sequences like `000000`, `00`, etc., during obliteration.

Although this is part of the secure deletion process, it creates **temporary visual garbage** in the same folder for a few milliseconds. Because Nautilus and Thunar UI views are extremely fast and reactive, they often render these intermediate fake filenames, causing visual flicker and UX degradation.

### The Camouflage Solution

A custom logical manipulation was developed:

1. The original file is instantly moved using a very fast `std::fs::rename`.
2. It is relocated into **virtual hidden subdirectories** created on the fly.
3. These directories are named like: `.~shred_RANDOMSID`.

Because they start with a dot (`.`), they are automatically hidden by Unix conventions and ignored by file manager views.

All the noisy operations of `shred -u` then occur invisibly in the background, resulting in a perfectly clean user experience with no visual artifacts.

---

## 4. No Heavy Third‑Party Dependencies and Native Dialogs

Typical extension pipelines rely on Gettext (`.mo/.po`) for translations and often pull in `gtk4-rs` just to display confirmation dialogs. This would dramatically increase the size and complexity of the final `.so` binary.

### Internal Micro i18n Engine

A lightweight internal i18n structure was implemented. By reading the system environment variable (`$LANG`), the extension selects pre‑computed localized strings for Portuguese, Spanish, or default English — without any gettext tooling or compilation overhead.

### Zenity Dialog Callbacks

Instead of linking against GTK/Libadwaita for dialogs, the extension spawns:

```bash
zenity
```

`zenity` is a small native C dialog tool that integrates perfectly with the current GTK theme and is preinstalled on most GNOME and modern Linux desktops.

The extension reads the returned **exit codes** (OK / Cancel) to decide whether to proceed with destruction.

This approach keeps the binary extremely small, avoids heavy dependencies, and maintains native visual integration with the user’s desktop environment.

---

This architecture allows Shred Extension Rs to remain minimal, fast, portable, and highly compatible with modern Linux desktop environments while avoiding the pitfalls of outdated bindings and bloated dependencies.
