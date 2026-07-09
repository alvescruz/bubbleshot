# Changelog

## [0.2.0] - 2026-07-09

### Changed
- Renderer reescrito: `renderer.rs` → `renderer/mod.rs` + `renderer/shapes.rs`
- `render_to_image()` agora retorna `Option<RgbaImage>` em vez de pânico
- Texto renderizado via `ab_glyph` outlines + `tiny_skia::PathBuilder` com agrupamento de contornos
- `imageproc` removido, `tiny-skia 0.11` como engine gráfica
- Render toolbar e interações do canvas extraídos em sub-funções

### Fixed
- Anti-aliasing e espessura das anotações ao salvar/copiar
- Contorno do texto preserva furos internos ('O', '8')
- Centralização do step number com métricas reais da fonte
- Alinhamento do text tool (baseline com ascent offset)

### Quality
- Zero warnings em `cargo clippy` (default + pedantic)
- `is_light_color()` extraído para `types.rs`
- `handle_canvas_interactions` e `render_toolbar` divididos em sub-funções
