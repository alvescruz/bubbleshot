# GEMINI.md - theoshot

## Project Identity
**theoshot** is a screen capture and annotation tool for Linux, built with Rust and egui. It leverages XDG Desktop Portals via `ashpd` for secure and platform-agnostic screenshot capabilities on Wayland and X11.

## Tech Stack
- **Language:** Rust (Edition 2024)
- **UI Framework:** `egui` / `eframe` (Immediate mode UI)
- **Graphics/Image:** `image`, `tiny-skia`, `ab_glyph`
- **Linux Integration:** `ashpd` (Portals), `arboard` (Clipboard)
- **Async Runtime:** `tokio`

## Architecture & Core Modules
- `src/main.rs`: Entry point, CLI mode handling (interactive vs. screen).
- `src/capture.rs`: Handles the screenshot portal request and initial image loading.
- `src/ui/`:
  - `app.rs`: Main `eframe` App implementation, event loop, tool interactions.
  - `painter.rs`: On-screen drawing via egui for each tool.
  - `renderer/mod.rs`: Orchestrates `render_to_image()` for save/copy.
  - `renderer/shapes.rs`: Individual shape renderers and glyph path builder.
  - `types.rs`: Shared structs, enums, helpers.
  - `utils.rs`: Coordinate transforms (`point_to_pixel`).
  - `components.rs`: Reusable egui widgets.

## Operational Directives (Agent Best Practices)

### 1. Development Workflow
- **Build:** `cargo build`
- **Check:** `cargo check` (fastest way to verify type safety)
- **Formatting:** `cargo fmt` (always run before proposing changes)
- **Running:** `cargo run -- [mode]` (modes: `interactive`, `screen`)

### 2. UI & Aesthetics
- **Immediate Mode:** Remember `egui` is immediate mode. Logic and UI are often coupled in `update()` calls.
- **Styling:** Adhere to existing component patterns in `src/ui/components.rs`.
- **Transparency:** The app uses a transparent, maximized, undecorated window for the overlay effect.

### 3. Image Processing
- Use `image::RgbaImage` for image data and final output.
- Render annotations via `tiny-skia` vector paths (stroke/fill with anti-aliasing).
- Text rendering uses `ab_glyph` outline extraction + `tiny_skia::PathBuilder`.
- When adding annotation tools, add on-screen drawing to `src/ui/painter.rs` and final render to `src/ui/renderer/shapes.rs`.

### 4. Linux/Wayland Constraints
- Portals are async. `capture_frame` must be awaited in a `tokio` context.
- Clipboard operations via `arboard` should be handled carefully to avoid blocking the UI thread if possible.

## Optimized Operation Strategy
- **Surgical Edits:** Use `replace` for targeted logic changes.
- **Parallel Analysis:** When investigating a feature, read `app.rs`, `painter.rs`, and `types.rs` in parallel to understand the full state flow.
- **Validation:** Since automated GUI testing is limited, prioritize adding unit tests for utility functions in `src/ui/utils.rs` or image logic in `src/ui/painter.rs`.
- **Error Handling:** Follow the pattern of returning `Result<T, String>` for high-level errors as seen in `capture.rs`.

## Standards & Conventions
- **Language:** English for code, comments, and documentation.
- **Async:** Use `tokio` for I/O and portal interactions.
- **Types:** Use explicit types in `types.rs` to maintain structural integrity across the UI modules.

## Communication Style
- **Directness:** Always be direct and concise. Avoid verbose explanations or conversational filler. Focus strictly on technical rationale and intent.
