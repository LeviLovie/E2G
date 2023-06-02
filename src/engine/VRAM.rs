use image::{Rgb};

pub const SIZE: usize = 256;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r: r, g: g, b: b }
    }
}

// #[derive(Copy, Clone)]
// pub struct Pixel {
//     pub x: usize,
//     pub y: usize,
//     pub color: Color,
// }

// impl Pixel {
//     pub fn new(x: usize, y: usize, color: Color) -> Self {
//         Pixel {
//             x: x,
//             y: y,
//             color: color,
//         }
//     }
// }

const FONT_LETTERS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz(|)~";
const LETTER_SIZE: usize = 8;
const IMAGE_WIDTH: i32 = 160;
const IMAGE_HEIGTH: i32 = 40;

#[derive(Copy, Clone)]
pub struct VRAM {
    pub VRAM: [Color; SIZE*SIZE],
}

impl VRAM {
    pub fn new() -> Self {
        VRAM {
            VRAM: [Color::new(0.0, 0.0, 0.0); SIZE*SIZE],
        }
    }

    // Get pixel from VRAM
    pub fn VRAM_get_pixel(&self, x: usize, y: usize) -> Color {
        let num = y * SIZE + x;
        self.VRAM[num]
    }

    // Set pixel in VRAM
    pub fn VRAM_set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= SIZE || y >= SIZE {
            return;
        }
        let num = y * SIZE + x;
        self.VRAM[num] = color;
    }

    // Set pixels rect in VRAM
    pub fn VRAM_set_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for i in y..y + height {
            for j in x..x + width {
                self.VRAM_set_pixel(j, i, color);
            }
        }
    }

    pub fn VRAM_write_text(&mut self, x: usize, y: usize, color: Color, text: &str) {
        let text_length = text.len();

        let image_data = include_bytes!("../../static/font2.png");
        let image: image::DynamicImage = image::load_from_memory(image_data).unwrap();
        let img = image.to_rgb32f();

        for i in 0..text_length {
            for j in 0..FONT_LETTERS.len() {
                if text.chars().nth(i) == FONT_LETTERS.chars().nth(j) {
                    let mut letter_x = (j * LETTER_SIZE) % 160;
                    let mut letter_y = 0;
                    for i in 0..(j * LETTER_SIZE) / 160 {
                        letter_y += LETTER_SIZE;
                    }

                    let color2 = &Rgb([0, 0, 0]);
                    for k in 0..LETTER_SIZE {
                        for l in 0..LETTER_SIZE {
                            let img_pixel = img.get_pixel(letter_x as u32 + l as u32, letter_y as u32 + k as u32);
                            if img_pixel == &Rgb([0.0, 0.0, 0.0]) {
                                self.VRAM_set_pixel(x + l + (i * LETTER_SIZE), y + k, color);
                            }
                        }
                    }
                }
            }
        }
    }

    // Set pixels circle in VRAM
    pub fn VRAM_set_circle(&mut self, x: usize, y: usize, radius: usize, color: Color) {
        let mut x = x as i32;
        let mut y = y as i32;
        let mut radius = radius as i32;

        let mut x1 = 0;
        let mut y1 = radius;
        let mut d = 3 - 2 * radius;

        while y1 >= x1 {
            self.VRAM_set_pixel((x + x1) as usize, (y - y1) as usize, color);
            self.VRAM_set_pixel((x + y1) as usize, (y - x1) as usize, color);
            self.VRAM_set_pixel((x + y1) as usize, (y + x1) as usize, color);
            self.VRAM_set_pixel((x + x1) as usize, (y + y1) as usize, color);
            self.VRAM_set_pixel((x - x1) as usize, (y + y1) as usize, color);
            self.VRAM_set_pixel((x - y1) as usize, (y + x1) as usize, color);
            self.VRAM_set_pixel((x - y1) as usize, (y - x1) as usize, color);
            self.VRAM_set_pixel((x - x1) as usize, (y - y1) as usize, color);

            if d < 0 {
                d += 4 * x1 + 6;
            } else {
                d += 4 * (x1 - y1) + 10;
                y1 -= 1;
            }

            x1 += 1;
        }
    }

    // Set pixels line in VRAM
    pub fn VRAM_set_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: Color) {
        let mut x1 = x1 as i32;
        let mut y1 = y1 as i32;
        let mut x2 = x2 as i32;
        let mut y2 = y2 as i32;

        let mut dx = x2 - x1;
        let mut dy = y2 - y1;

        if dx.abs() > dy.abs() {
            let mut x = x1;
            let mut y = y1;

            let mut step = 1;
            if dx < 0 {
                step = -1;
                dx = -dx;
            }

            let mut p = 2 * dx - dy;
            while x != x2 {
                self.VRAM_set_pixel(x as usize, y as usize, color);

                if p > 0 {
                    y += 1;
                    p -= 2 * dx;
                }

                x += step;
                p += 2 * dy;
            }
        } else {
            let mut x = x1;
            let mut y = y1;

            let mut step = 1;
            if dy < 0 {
                step = -1;
                dy = -dy;
            }

            let mut p = 2 * dy - dx;
            while y != y2 {
                self.VRAM_set_pixel(x as usize, y as usize, color);

                if p > 0 {
                    x += 1;
                    p -= 2 * dy;
                }

                y += step;
                p += 2 * dx;
            }
        }
    }
}
