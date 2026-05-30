# Development

Thank you for your interest in contributing to **theoshot**!

## Project Structure

- **`src/main.rs`**: Entry point. Handles CLI arguments and decides between `interactive` (with UI) and `screen` (direct save/copy) capture.
- **`src/capture.rs`**: The core of the Linux integration. Uses `ashpd` to talk to the `org.freedesktop.portal.Screenshot` portal.
- **`src/ui/`**: All UI-related logic using `egui`.
    - **`app.rs`**: The main application state and event loop.
    - **`painter.rs`**: Low-level drawing functions for each tool.
    - **`renderer.rs`**: Logic to combine the screenshot buffer with the user annotations into a final `RgbaImage`.

## Architecture Flow

1.  **Request:** The app requests a screenshot from the XDG Portal.
2.  **Response:** The OS (GNOME) captures the screen and returns a file descriptor.
3.  **Loading:** Theoshot loads this into an `image::RgbaImage`.
4.  **UI:** The image is displayed as a background texture in a maximized, transparent window.
5.  **Interaction:** User draws `Shape` objects on top.
6.  **Finalize:** On Save/Copy, the `renderer` "bakes" the shapes into the background image pixels.

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
