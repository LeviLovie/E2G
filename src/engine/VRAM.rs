pub const SIZE: usize = 128;

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
const LETTERS_IN_LINE: usize = 20;
const LETTER_SIZE: usize = 8;
const LETTERS: [&str; FONT_LETTERS.len()] = 

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
        let image_data = include_bytes!("../../static/font.png");

        for i in 0..text_length {
            let mut letter_index = 0;
            for j in 0..FONT_LETTERS.len() {
                if text.chars().nth(i).unwrap() == FONT_LETTERS.chars().nth(j).unwrap() {
                    letter_index = j;
                    break;
                }
            }

            println!("letter_index: {}", letter_index);

            let mut letter_x = (letter_index % LETTERS_IN_LINE) * LETTER_SIZE;
            let mut letter_y = (letter_index / LETTERS_IN_LINE) * LETTER_SIZE;

            println!("letter_x: {}", letter_x);
            println!("letter_y: {}", letter_y);

            let mut letter: [i32; LETTER_SIZE*LETTER_SIZE] = [0; LETTER_SIZE*LETTER_SIZE];
            for i in 0..LETTER_SIZE {
                for j in 0..LETTER_SIZE {
                    let mut pixel = 0;
                    if LETTERS.chars().nth(letter_y * LETTER_SIZE + i + letter_x * LETTER_SIZE + j).unwrap() == '1' {
                        pixel = 1;
                    }
                    letter[(i * LETTER_SIZE) + j] = pixel;
                }
            }

            for i in 0..LETTER_SIZE {
                for j in 0..LETTER_SIZE {
                    print!("{}", letter[i * LETTER_SIZE + j]);
                }
                println!();
            }

            for i in 0..LETTER_SIZE {
                for j in 0..LETTER_SIZE {
                    if i <= SIZE && j <= SIZE {
                        if letter[i * LETTER_SIZE + j] == 1 {
                            self.VRAM_set_pixel(x + j, y + i, color);
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
