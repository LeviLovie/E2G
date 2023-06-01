#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

mod engine;
mod compiler;
mod ERR;
mod configs;

use configs::*;
use std::env;

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
    
    let mut compiler = compiler::compiler::Compiler::new();
    compiler.Run();
}
