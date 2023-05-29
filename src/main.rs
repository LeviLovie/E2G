#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

mod GFX;
mod configs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "migrate" {configs::Migrate(); return;}
    }
    let boot_conf = configs::Get_boot_conf();
    let mut gfx = GFX::GFX::new(boot_conf.pixel_size);
}
