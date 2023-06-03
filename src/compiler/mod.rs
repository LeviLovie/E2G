pub mod draws;
pub mod boot;
pub mod compiler;
pub mod consts;

use std::{sync::{Arc, Mutex}, thread};
use crate::{engine, configs};
use crate::ERR;
use consts::*;
use boot::*;
use compiler::*;

use self::draws::fill_bg;
pub struct Compiler {
    pub app: String,
}

impl Compiler {
    pub fn new() -> Self {
        let vram_mut = Arc::new(Mutex::new(engine::VRAM::VRAM::new()));
        Compiler {
            app: configs::Get_boot_conf().boot,
        }
    }

    pub fn Run(&mut self) {
        ERR::err_catch(ERR::Err::new(format!("Booting {:?}", self.app).as_str(), ERR::LEVEL_INFO));
        let mut gfx = engine::GFX::GFX::new(configs::Get_boot_conf().pixel_size);
        let vram_mut = Arc::clone(&gfx.VRAM);
        let app = self.app.clone();
        
        fill_bg(COLOR_BLACK, &vram_mut);
        thread::spawn(move || {
            start_animation(&vram_mut);
            Compile_app(app, &vram_mut);
            // start_text(&vram_mut);
            // wait_write_text(0, 41, COLOR_GRAY, "> ", 500, &vram_mut);
        });

        gfx.Run();
    }
}
