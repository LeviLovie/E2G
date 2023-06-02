use crate::{engine};

// 16 colors
pub const COLOR_WHITE:        engine::VRAM::Color = engine::VRAM::Color{r: 1.0, g: 1.0, b: 1.0};
pub const COLOR_BLACK:        engine::VRAM::Color = engine::VRAM::Color{r: 0.0, g: 0.0, b: 0.0};
pub const COLOR_RED:          engine::VRAM::Color = engine::VRAM::Color{r: 1.0, g: 0.0, b: 0.0};
pub const COLOR_GREEN:        engine::VRAM::Color = engine::VRAM::Color{r: 0.0, g: 1.0, b: 0.0};
pub const COLOR_BLUE:         engine::VRAM::Color = engine::VRAM::Color{r: 0.0, g: 0.0, b: 1.0};
pub const COLOR_YELLOW:       engine::VRAM::Color = engine::VRAM::Color{r: 1.0, g: 1.0, b: 0.0};
pub const COLOR_MAGENTA:      engine::VRAM::Color = engine::VRAM::Color{r: 1.0, g: 0.0, b: 1.0};
pub const COLOR_CYAN:         engine::VRAM::Color = engine::VRAM::Color{r: 0.0, g: 1.0, b: 1.0};
pub const COLOR_GRAY:         engine::VRAM::Color = engine::VRAM::Color{r: 0.5, g: 0.5, b: 0.5};
pub const COLOR_DARK_RED:     engine::VRAM::Color = engine::VRAM::Color{r: 0.5, g: 0.0, b: 0.0};
pub const COLOR_DARK_GREEN:   engine::VRAM::Color = engine::VRAM::Color{r: 0.0, g: 0.5, b: 0.0};
pub const COLOR_DARK_BLUE:    engine::VRAM::Color = engine::VRAM::Color{r: 0.0, g: 0.0, b: 0.5};
pub const COLOR_DARK_YELLOW:  engine::VRAM::Color = engine::VRAM::Color{r: 0.5, g: 0.5, b: 0.0};
pub const COLOR_DARK_MAGENTA: engine::VRAM::Color = engine::VRAM::Color{r: 0.5, g: 0.0, b: 0.5};
pub const COLOR_DARK_CYAN:    engine::VRAM::Color = engine::VRAM::Color{r: 0.0, g: 0.5, b: 0.5};
pub const COLOR_DARK_GRAY:    engine::VRAM::Color = engine::VRAM::Color{r: 0.25, g: 0.25, b: 0.25};

pub const COLORS: [engine::VRAM::Color; 16] = [
    COLOR_BLACK,
    COLOR_WHITE,
    COLOR_RED,
    COLOR_GREEN,
    COLOR_BLUE,
    COLOR_YELLOW,
    COLOR_MAGENTA,
    COLOR_CYAN,
    COLOR_GRAY,
    COLOR_DARK_RED,
    COLOR_DARK_GREEN,
    COLOR_DARK_BLUE,
    COLOR_DARK_YELLOW,
    COLOR_DARK_MAGENTA,
    COLOR_DARK_CYAN,
    COLOR_DARK_GRAY,
];
