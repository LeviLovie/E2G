// use rand::{thread_rng, Rng};
// use std::{sync::{Arc, Mutex}, thread, time::Duration};
use std::{sync::{Arc, Mutex}, thread};
use crate::{engine, configs};
use crate::compiler::*;
use crate::ERR;
use draws::*;
use consts::*;
use boot::*;
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
        ERR::err_catch(ERR::Err::new(format!("Booting {:?}", self.app).as_str(), ERR::LEVEL_INFO));
        let mut gfx = engine::GFX::GFX::new(configs::Get_boot_conf().pixel_size);
        let vram_mut = Arc::clone(&gfx.VRAM);
        thread::spawn(move || {
            start_animation(&vram_mut);
            start_text(&vram_mut);
            wait_write_text(0, 41, COLOR_GRAY, "> ", 500, &vram_mut);
        });

        gfx.Run();
    }
}
