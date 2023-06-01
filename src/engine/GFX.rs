use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::VRAM;
use crate::ERR;

thread_local!(
    static GLOBAL: RefCell<Option<Window>> = RefCell::new(None);
);

fn check_update_display(){
    GLOBAL.with(|global|{
        if let Some(win) = &*global.borrow() {
            win.queue_draw();
        }
    })
}

pub struct GFX {
    pub VRAM: Arc<Mutex<VRAM::VRAM>>,
    pub screen_pixel_size: usize,
    pub name: String,
}

impl GFX {
    pub fn new(pixel_size: usize) -> Self {
        let vram_mut = Arc::new(Mutex::new(VRAM::VRAM::new()));
        GFX {
            VRAM: vram_mut,
            screen_pixel_size: pixel_size,
            name: "E2G".to_string(),
        }
    }

    pub fn Run(&self) {
        gtk::init().expect("Failed to initialize GTK");
        let window: Window = Window::new(WindowType::Toplevel);
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

        window.add(&drawing_area);
        window.show_all();

        let vram_mut = Arc::clone(&self.VRAM);

        drawing_area.connect_draw(move |_, cr| {
            let mut vram = vram_mut.lock().unwrap();
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

            drop(vram);

            Inhibit(false)
        });

        window.connect_event(|w, _| {
            w.queue_draw();
            Inhibit(false)
        });

        GLOBAL.with(|global|{
            *global.borrow_mut() = Some(window);
        });

        glib::timeout_add(Duration::from_millis(100), move || {
            check_update_display();
            glib::Continue(true)
        });

        gtk::main();
    }
}
