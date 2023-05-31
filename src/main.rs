#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

mod engine;
mod configs;

use std::env;
use configs::*;

// use std::f64::consts::PI;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "migrate" {Migrate();return;}
    }
    let boot_conf = Get_boot_conf();
    let mut gfx = engine::GFX::GFX::new(boot_conf.pixel_size);
    
    // for i in 0..engine::VRAM::SIZE {
    //     // engine::VRAM::Pixel::new(j, i, engine::VRAM::Color::new((PI / (i/10) as f64).sin(), (PI / (i/10) as f64).cos(), 0.0))
    //     let color = engine::VRAM::Color::new((PI / (i/10) as f64).sin(), (PI / (i/10) as f64).cos(), 0.0);
    //     gfx.VRAM.VRAM_set_line(0, i, engine::VRAM::SIZE - 1, i - 1, color)
    // }

    gfx.VRAM.VRAM_set_rect(10, 10, 108, 108, engine::VRAM::Color::new(256.0, 0.0, 0.0));
    gfx.VRAM.VRAM_set_rect(20, 20, 88, 88, engine::VRAM::Color::new(0.0, 256.0, 0.0));
    gfx.VRAM.VRAM_set_rect(30, 30, 68, 68, engine::VRAM::Color::new(0.0, 0.0, 256.0));
    gfx.VRAM.VRAM_set_rect(40, 40, 48, 48, engine::VRAM::Color::new(256.0, 256.0, 0.0));
    gfx.VRAM.VRAM_set_rect(50, 50, 28, 28, engine::VRAM::Color::new(256.0, 0.0, 256.0));
    gfx.VRAM.VRAM_set_rect(60, 60, 8, 8, engine::VRAM::Color::new(0.0, 256.0, 256.0));

    gfx.VRAM.VRAM_set_circle(64, 64, 50, engine::VRAM::Color::new(256.0, 256.0, 256.0));

    gfx.VRAM.VRAM_set_line(10, 20, 108, 98, engine::VRAM::Color::new(256.0, 256.0, 256.0));

    gfx.Run();
}