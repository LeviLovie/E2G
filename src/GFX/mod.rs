mod VRAM;

pub struct GFX {
    pub VRAM: VRAM::VRAM,
    pub screen_pixel_size: usize,
}

impl GFX {
    pub fn new(pixel_size: usize) -> Self {
        GFX {
            VRAM: VRAM::VRAM::new(),
            screen_pixel_size: pixel_size,
        }
    }
}