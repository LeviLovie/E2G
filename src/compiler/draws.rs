use std::{sync::{Arc, Mutex}, thread, time::Duration};
use crate::{engine};

pub fn write_text(x: usize, y: usize, color: engine::VRAM::Color, text: &str, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    let mut vram = vram_mut.lock().unwrap();
    vram.VRAM_write_text(x, y, color, text);
    drop(vram);
}

pub fn wait_write_text(x: usize, y: usize, color: engine::VRAM::Color, text: &str, delay: u64, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    wait(delay);
    let mut vram = vram_mut.lock().unwrap();
    vram.VRAM_write_text(x, y, color, text);
    drop(vram);
}

pub fn draw_pixel(x: usize, y: usize, color: engine::VRAM::Color, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    let mut vram = vram_mut.lock().unwrap();
    vram.VRAM_set_pixel(x, y, color);
    drop(vram);
}

pub fn draw_rect(x: usize, y: usize, w: usize, h: usize, color: engine::VRAM::Color, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    let mut vram = vram_mut.lock().unwrap();
    vram.VRAM_set_rect(x, y, w, h, color);
    drop(vram);
}

pub fn wait_draw_rect(x: usize, y: usize, w: usize, h: usize, color: engine::VRAM::Color, delay: u64, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    wait(delay);
    let mut vram = vram_mut.lock().unwrap();
    vram.VRAM_set_rect(x, y, w, h, color);
    drop(vram);
}

pub fn fill_bg(color: engine::VRAM::Color, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    let mut vram = vram_mut.lock().unwrap();
    vram.VRAM_set_rect(0, 0, engine::VRAM::SIZE, engine::VRAM::SIZE, color);
    drop(vram);
}

pub fn wait_fill_bg(color: engine::VRAM::Color, delay: u64) {
    wait(delay);
    let mut vram = engine::VRAM::VRAM::new();
    vram.VRAM_set_rect(0, 0, engine::VRAM::SIZE, engine::VRAM::SIZE, color);
    drop(vram);
}

pub fn wait(delay: u64) {
    thread::sleep(Duration::from_millis(delay));
}