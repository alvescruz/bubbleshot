---
description: >-
  Contribute to theoshot development. Learn about the Rust project structure,
  egui UI architecture, tiny-skia renderer, XDG Portal integration, how to build
  from source, and testing guidelines.
tags:
  - development
  - contributing
  - rust
  - architecture
  - build
---

# Development

Thank you for your interest in contributing to **theoshot**!

## Project Structure

- **`src/main.rs`**: Entry point. Handles CLI arguments and decides between `interactive` (with UI) and `screen` (direct save/copy) capture.
- **`src/capture.rs`**: The core of the Linux integration. Uses `ashpd` to talk to the `org.freedesktop.portal.Screenshot` portal.
- **`src/ui/`**: All UI-related logic using `egui`.
    - **`app.rs`**: Main application state, event loop, and tool interactions.
    - **`painter.rs`**: On-screen drawing via egui for each tool.
    - **`renderer/`**: Render pipeline for save/copy.
        - **`mod.rs`**: Orchestrates `render_to_image()`, returns `Option<RgbaImage>`.
        - **`shapes.rs`**: Individual shape renderers (rect, circle, step, pen, arrow, blur, text) and glyph path builder.
    - **`types.rs`**: Shared structs, enums, and helper predicates (`is_light_color`).
    - **`utils.rs`**: Coordinate transforms (`point_to_pixel`, `get_arrow_points`).
    - **`components.rs`**: Reusable egui widgets (`styled_button`, `action_button`).

## Architecture Flow

1.  **Request:** The app requests a screenshot from the XDG Portal.
2.  **Response:** The OS (GNOME) captures the screen and returns a file descriptor.
3.  **Loading:** Theoshot loads this into an `image::RgbaImage`.
4.  **UI:** The image is displayed as a background texture in a maximized, transparent window.
5.  **Interaction:** User draws `Shape` objects on top.
6.  **Finalize:** On Save/Copy, the renderer "bakes" shapes into the background using **tiny-skia** (vector paths with anti-aliasing, sub-pixel precision) and **ab_glyph** outline extraction for text. Output is full native resolution RGBA (`RgbaImage`).

## Prerequisites

On Ubuntu/Debian:
```bash
sudo apt install libgtk-3-dev libpipewire-0.3-dev libdbus-1-dev pkg-config
```

## Building and Running

To run the project in development mode:
```bash
cargo run -- interactive
```

To build for production:
```bash
cargo build --release
```

## 🧪 Testing & Quality

Currently, the project focuses on integration tests via manual runs. We are looking to add:
- Unit tests for shape bounding box logic in `types.rs`.
- Unit tests for image coordinate transformations in `utils.rs`.

Always run `cargo fmt` before submitting a Pull Request.

## Contributing

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a Pull Request.
