use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::VRAM;
use crate::ERR;

pub struct GFX {
    pub VRAM: VRAM::VRAM,
    pub screen_pixel_size: usize,
    pub name: String,
}

impl GFX {
    pub fn new(pixel_size: usize) -> Self {
        GFX {
            VRAM: VRAM::VRAM::new(),
            screen_pixel_size: pixel_size,
            name: "E2G".to_string(),
        }
    }

    pub fn Run(&self) {
        gtk::init().expect("Failed to initialize GTK");
        let window = Window::new(WindowType::Toplevel);
        window.set_title(self.name.as_str());
        window.set_default_size(
            (VRAM::SIZE * self.screen_pixel_size) as i32,
            (VRAM::SIZE * self.screen_pixel_size) as i32,
        );
        window.set_position(gtk::WindowPosition::Center);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let drawing_area = DrawingArea::new();
        let pixel_size = self.screen_pixel_size;
        let vram_copy = Arc::new(Mutex::new(self.VRAM.clone()));

        window.add(&drawing_area);
        window.show_all();

        let vram_copy_thread = vram_copy.clone();
        thread::spawn(move || {
            loop {
                let mut vram = vram_copy_thread.lock().unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        drawing_area.connect_draw(move |_, cr| {
            let vram_copy = Arc::clone(&vram_copy);
            let vram = vram_copy.lock().unwrap();
            let mut err;
            for i in 0..VRAM::SIZE * VRAM::SIZE {
                let x = i % VRAM::SIZE;
                let y = i / VRAM::SIZE;
                let color = vram.VRAM_get_pixel(x, y);
                cr.set_source_rgb(color.r, color.g, color.b);
                cr.rectangle(
                    (x * pixel_size) as f64,
                    (y * pixel_size) as f64,
                    pixel_size as f64,
                    pixel_size as f64,
                );
                err = cr.fill();
                if err != Ok(()) {
                    ERR::err_catch(ERR::Err::new(
                        "Failed to draw pixel, GTK error; {}",
                        ERR::LEVEL_ERR_LOG,
                    ));
                }
            }
            Inhibit(false)
        });

        gtk::main();
    }
}
