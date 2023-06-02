use std::{sync::{Arc, Mutex}};
use crate::{engine::{self, VRAM::Color}};
use crate::compiler::*;
use consts::*;
use draws::*;

pub fn start_text(vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    wait_write_text(0, 1, COLOR_WHITE, "Allocating memory (RAM, VRAM)", 100, &vram_mut);
    wait_write_text(0, 9, COLOR_WHITE, "Reading boot ini", 100, &vram_mut);
    wait_write_text(0, 17, COLOR_WHITE, "Loading OS", 100, &vram_mut);
    wait_write_text(0, 25, COLOR_WHITE, "Booting", 100, &vram_mut);
}

pub fn start_animation(vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    let mut color_id = 0;
    let mut colors: [Color; 8] = [COLOR_BLACK; 8];

    for k in 0..colors.len() * 2 {
        for i in 0..engine::VRAM::SIZE / 2 {
            for j in 0..engine::VRAM::SIZE / 2 {
                draw_pixel(j * 2, i * 2, colors[color_id], vram_mut);
                draw_pixel(j * 2, i * 2, colors[color_id], vram_mut);
                draw_pixel(j * 2 + 1, i * 2, colors[color_id], vram_mut);
                draw_pixel(j * 2 + 1, i * 2, colors[color_id], vram_mut);

                draw_pixel(j * 2, i * 2 + 1, colors[color_id], vram_mut);
                draw_pixel(j * 2, i * 2 + 1, colors[color_id], vram_mut);
                draw_pixel(j * 2 + 1, i * 2 + 1, colors[color_id], vram_mut);
                draw_pixel(j * 2 + 1, i * 2 + 1, colors[color_id], vram_mut);

                color_id += 1;
                if color_id >= colors.len() {
                    color_id = 0;
                }
            }
            color_id += 1;
            if color_id >= colors.len() {
                color_id = 0;
            }
        }
        if k < colors.len() {
            colors[k] = COLORS[k];
        } else {
            colors[k - colors.len()] = COLOR_BLACK;
        }
        wait(75)
    }
    fill_bg(COLOR_BLACK, vram_mut);
    wait(1000)
}
