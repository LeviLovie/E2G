#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

mod engine;
mod compiler;
mod ERR;
mod configs;

use configs::*;
use std::env;
use std::io::{stdout, Write};
use termion::{clear, cursor};

static mut RESTART: bool = true;

fn main() {
    let mut stdout = stdout();write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap(); stdout.flush().unwrap();
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if      args[1] == "delete" {Delete();return;}
        else if args[1] == "check"  {Check(false);return;}
        else if args[1] == "patch"  {Check(true);return;}
        else if args[1] == "migrate"  {Delete();Check(true);return;}
    }

    // loop {
    //     unsafe {
    //         if RESTART {
    //             RESTART = false;
    //             thread::spawn(move || {
    Check(false);
    let mut compiler = compiler::Compiler::new();
    compiler.Run();
    
    //             });
    //         }
    //     }
    // }
}
