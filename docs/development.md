# Development

Thank you for your interest in contributing to **theoshot**!

## Project Structure

- `src/main.rs`: Entry point and CLI handling.
- `src/capture.rs`: Portal interaction and image acquisition.
- `src/ui/`: All UI-related logic (egui).
  - `app.rs`: Main application loop.
  - `painter.rs`: Drawing and annotation logic.

## Prerequisites

- **Rust:** Edition 2024.
- **Dependencies:** `libgtk-3-dev`, `libpipewire-0.3-dev`, `libdbus-1-dev`, `pkg-config`.

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

## Contributing

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Ensure your code is formatted with `cargo fmt`.
4. Submit a Pull Request.
