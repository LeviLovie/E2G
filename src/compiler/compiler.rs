// use rand::{thread_rng, Rng};
// use std::{sync::{Arc, Mutex}, thread, time::Duration};
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use crate::{engine, configs};
use crate::compiler::*;

pub struct Compiler {
    pub app: String,
}

impl Compiler {
    pub fn new() -> Self {
        let vram_mut = Arc::new(Mutex::new(engine::VRAM::VRAM::new()));
        Compiler {
            app: configs::Get_boot_conf().os,
        }
    }

    pub fn Run(&mut self) {
        println!("Booting {:?}", self.app);
        let mut gfx = engine::GFX::GFX::new(configs::Get_boot_conf().pixel_size);

        let vram_mut = Arc::clone(&gfx.VRAM);
        thread::spawn(move || {
            let mut vram = vram_mut.lock().unwrap();
            vram.VRAM_set_rect(0, 0, engine::VRAM::SIZE, engine::VRAM::SIZE, consts::COLOR_BLACK);
            drop(vram);

            // thread::sleep(Duration::from_millis(2000));

            // let mut vram = vram_mut.lock().unwrap();
            // vram.VRAM_set_rect(0, 0, engine::VRAM::SIZE, engine::VRAM::SIZE, consts::COLOR_DARK_BLUE);
            // drop(vram);

            // thread::sleep(Duration::from_millis(2000));

            let mut vram = vram_mut.lock().unwrap();
            vram.VRAM_write_text(0, 0, consts::COLOR_CYAN, "!");
            drop(vram);

            thread::sleep(Duration::from_millis(2000));
        });

        gfx.Run();
    }
}
