# theoshot

Welcome to the documentation for **theoshot**, a specialized screen capture and annotation tool designed for modern Linux environments.

!!! info "Target Environment"
    **theoshot** is specifically built and tested for **Linux** using the **Wayland** display protocol and the **GNOME** desktop environment. It leverages XDG Desktop Portals to ensure secure and native integration.

Built with Rust and egui, theoshot provides a fast, memory-safe way to take screenshots and annotate them immediately.

---

## 📸 See it in Action

![theoshot interactive mode and annotation tools](assets/screenshots/main-demo.gif)

---

## 🚀 Key Features

- **XDG Desktop Portal Integration:** Secure and platform-agnostic screenshotting (optimized for GNOME/Wayland).
- **Immediate Annotation:** Built-in tools to draw, highlight, and add text to your captures.
- **Modern Tech Stack:** Written in Rust 2024 for maximum performance.
- **Multiple Modes:** Interactive overlay for precision or quick full-screen capture.
- **Tutorial Friendly:** Includes a "Steps" tool to number parts of your screen easily.

## ❓ Why theoshot?

Most screenshot tools for Linux are either legacy (designed for X11) or very basic. **theoshot** was born to:
1.  Work perfectly with **Wayland** and Portals without "hacks".
2.  Provide an **integrated annotation** flow—no more opening a second app just to draw an arrow.
3.  Be **blazing fast** thanks to the Rust backend and immediate-mode UI.

## 🗺️ Roadmap

- [ ] Support for KDE Plasma Portals.
- [ ] Customizable keyboard shortcuts.
- [ ] Multi-monitor support improvements.
- [ ] Export to more formats (WebP, JPG).
- [ ] Color picker for custom hex colors.

## Quick Start

To install theoshot on your GNOME/Wayland system:

```bash
curl -sSL https://raw.githubusercontent.com/alvescruz/theoshot/main/install.sh | sudo bash
```

Then launch the interactive tool:

```bash
theoshot interactive
```

Check the [Usage](usage.md) section to learn how to bind this to your PrintScreen key!
