#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

mod engine;
mod ERR;
mod configs;

use rand::{thread_rng, Rng};
use std::{env, sync::{Arc}, thread, time::Duration};
use configs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if      args[1] == "delete" {Delete();return;}
        else if args[1] == "check"  {Check(false);return;}
        else if args[1] == "patch"  {Check(true);return;}
        else if args[1] == "migrate"  {Delete();Check(true);return;}
    }
    Check(false);
    let boot_conf = Get_boot_conf();
    let mut gfx = engine::GFX::GFX::new(boot_conf.pixel_size);

    let vram_mut = Arc::clone(&gfx.VRAM);

    let mut vram = vram_mut.lock().unwrap();
    vram.VRAM_set_rect(10, 10, 108, 108, engine::VRAM::Color { r: 255.0, g: 0.0, b: 0.0 });
    drop(vram);

    thread::spawn(move || {
        let mut rng = thread_rng();
        loop {
            let x = 20 + rng.gen_range(0..98);
            let y = 20 + rng.gen_range(0..98);
            let color = engine::VRAM::Color {r: 0.0, g: 0.0, b: 0.0};

            let mut vram = vram_mut.lock().unwrap();
            vram.VRAM_set_pixel(x, y, color);
            drop(vram);

            thread::sleep(Duration::from_millis(100));
        }
    });

    gfx.Run();
}
