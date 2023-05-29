const SIZE: usize = 128;

pub struct VRAM {
    VRAM: [[i32; SIZE]; SIZE],
}

impl VRAM {
    pub fn new() -> Self {
        VRAM {
            VRAM: [[0; SIZE]; SIZE],
        }
    }

    // Get VRAM array
    pub fn VRAM_get(&self) -> &[[i32; SIZE]; SIZE] {
        &self.VRAM
    }

    // Set VRAM array
    pub fn VRAM_ste(&mut self, new_VRAM: [[i32; SIZE]; SIZE]) {
        self.VRAM = new_VRAM;
    }

    // Get pixel from VRAM
    pub fn VRAM_get_pixel(&self, x: usize, y: usize) -> i32 {
        self.VRAM[y][x]
    }

    // Set pixel in VRAM
    pub fn VRAM_set_pixel(&mut self, x: usize, y: usize, value: i32) {
        self.VRAM[y][x] = value;
    }

    // Set pixels rect in VRAM
    pub fn VRAM_set_rect(&mut self, x: usize, y: usize, width: usize, height: usize, value: i32) {
        for i in y..y + height {
            for j in x..x + width {
                self.VRAM[i][j] = value;
            }
        }
    }

    // Set pixels circle in VRAM
    pub fn VRAM_set_circle(&mut self, x: usize, y: usize, radius: usize, value: i32) {
        for i in y - radius..y + radius {
            for j in x - radius..x + radius {
                if (i - y) * (i - y) + (j - x) * (j - x) <= radius * radius {
                    self.VRAM[i][j] = value;
                }
            }
        }
    }

    // Set pixels line in VRAM
    pub fn VRAM_set_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, value: i32) {
        let mut x1 = x1 as i32;
        let mut y1 = y1 as i32;
        let mut x2 = x2 as i32;
        let mut y2 = y2 as i32;

        let mut steep = false;
        if (x1 - x2).abs() < (y1 - y2).abs() {
            std::mem::swap(&mut x1, &mut y1);
            std::mem::swap(&mut x2, &mut y2);
            steep = true;
        }

        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut y1, &mut y2);
        }

        let dx = x2 - x1;
        let dy = y2 - y1;
        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = y1;

        for x in x1..x2 {
            if steep {
                self.VRAM[y as usize][x as usize] = value;
            } else {
                self.VRAM[x as usize][y as usize] = value;
            }

            error2 += derror2;

            if error2 > dx {
                y += if y2 > y1 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }
}
