use std::{sync::{Arc, Mutex}, time::{Duration, Instant}, thread};
use std::fs::File;
use std::io::Read;
use colored::Colorize;
use chrono::prelude::*;

use crate::{engine, configs};
use super::consts::*;
use super::draws::*;
use crate::ERR;

pub fn Compile_app(app: String, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    let mut lines = read_to_array(app);

    // Delete useless lines
    lines = remove_comments(lines);

    // Split lines to start and update functions
    let mut lines_start: Vec<String> = Vec::new();
    let mut is_lines_start: bool = false;
    let mut lines_start_id: usize = 0;
    let mut lines_update: Vec<String> = Vec::new();
    let mut is_lines_update: bool = false;
    let mut lines_update_id: usize = 0;
    for i in 0..lines.len() {
        if lines[i] == "fn start" {is_lines_start = true; lines_start_id = i + 2; continue;}
        if is_lines_start {
            if lines[i] == "endfn" {is_lines_start = false; continue;}
            lines_start.push(lines[i].clone());
        }

        if lines[i] == "fn update" {is_lines_update = true; lines_update_id = i + 2; continue;}
        if is_lines_update {
            if lines[i] == "endfn" {is_lines_update = false; continue;}
            lines_update.push(lines[i].clone());
        }
    }

    // Compile code
    compile_lines(lines_start, lines_start_id, vram_mut);
    let target_tps = 10;
    let update_interval = Duration::from_secs_f64(1.0 / target_tps as f64);
    loop {
        let start_time = Instant::now();
        // compile_lines(lines_update.clone(), lines_update_id, vram_mut);
        let elapsed_time = start_time.elapsed();
        if elapsed_time < update_interval {
            let sleep_time = update_interval - elapsed_time;
            thread::sleep(sleep_time);
        }
    }
}

fn compile_lines(lines: Vec<String>, start_id: usize, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    for i in 0..lines.len() {
        compile_line(lines[i].clone(), start_id + i, vram_mut);
    }
}

fn compile_line(line: String, id: usize, vram_mut: &Arc<Mutex<engine::VRAM::VRAM>>) {
    let mut line_post = "";
    if line.starts_with("log ") {
        line_post = line.strip_prefix("log ".to_string().as_str()).unwrap();
        if line_post.starts_with("`") && line_post.ends_with("`") {
            let line_post = line_post.strip_prefix("`").unwrap();
            let line_post = line_post.strip_suffix("`").unwrap();
            let mut id: String = format!("#{:0>6}", id).to_string();
            println!("{:<30}{:<30}{}", Local::now().format("%Y-%m-%dT%H:%M:%S").to_string().blue().to_string(), id.to_string().purple().to_string(), line_post);
        } else {
            ERR::err_catch(ERR::Err::new(format!("Can't be find information to log (must be string in ``); Line {:?}", id).as_str(), ERR::LEVEL_ERR_LOG));
            return;
        }
    } else if line.starts_with("print ") {
        line_post = line.strip_prefix("print ".to_string().as_str()).unwrap();
        let mut args: Vec<String> = line_post.split(":").map(|s: &str| s.to_string()).collect();
        if args.len() != 4 {
            ERR::err_catch(ERR::Err::new(format!("Not corect amount of args (must be 4 (\":\" for split)); Line {:?}", id).as_str(), ERR::LEVEL_ERR_LOG));
            return;
        }
        let mut x: i32 = args[0].parse().unwrap();
        let mut y: i32 = args[1].parse().unwrap();
        let mut color: i32 = args[2].parse().unwrap();
        if color < 1 || color > COLORS.len() as i32 {
            ERR::err_catch(ERR::Err::new(format!("Not corect color (must be 1-16); Line {:?}", id).as_str(), ERR::LEVEL_ERR_LOG));
            return;
        }
        let mut text: String = "".to_string();
        if args[3].starts_with("`") && line_post.ends_with("`") {
            text = args[3].strip_prefix("`").unwrap().to_string();
            text = text.strip_suffix("`").unwrap().to_string();
        } else {
            ERR::err_catch(ERR::Err::new(format!("Can't be find information to arg #4 (must be string in ``); Line {:?}", id).as_str(), ERR::LEVEL_ERR_LOG));
            return;
        }
        write_text(x as usize, y as usize, COLORS[color as usize - 1], text.as_str(), vram_mut)
    } else if line.starts_with("rect ") {
        line_post = line.strip_prefix("rect ".to_string().as_str()).unwrap();
        let mut args: Vec<String> = line_post.split(":").map(|s: &str| s.to_string()).collect();
        if args.len() != 5 {
            ERR::err_catch(ERR::Err::new(format!("Not corect amount of args (must be 4 (\":\" for split)); Line {:?}", id).as_str(), ERR::LEVEL_ERR_LOG));
            return;
        }
        let mut x: i32 = args[0].parse().unwrap();
        let mut y: i32 = args[1].parse().unwrap();
        let mut w: i32 = args[2].parse().unwrap();
        let mut h: i32 = args[3].parse().unwrap();
        let mut color: i32 = args[4].parse().unwrap();
        draw_rect(x as usize, y as usize, w as usize, h as usize, COLORS[color as usize - 1], vram_mut)
    } else if line.starts_with("fill ") {
        line_post = line.strip_prefix("fill ".to_string().as_str()).unwrap();
        if line_post == "" {
            ERR::err_catch(ERR::Err::new(format!("Not corect amount of args (must be 1); Line {:?}", id).as_str(), ERR::LEVEL_ERR_LOG));
            return;
        }
        let mut color: i32 = line_post.parse().unwrap();
        if color < 1 || color > COLORS.len() as i32 {
            ERR::err_catch(ERR::Err::new(format!("Not corect color (must be 1-16); Line {:?}", id).as_str(), ERR::LEVEL_ERR_LOG));
            return;
        }
        fill_bg(COLORS[color as usize - 1], vram_mut)
    } else {
        if line == "" {return;}
        ERR::err_catch(ERR::Err::new(format!("Can't be indentificated procedure (\"#\" or \"\\\\\" for comment); Line {:?}", id).as_str(), ERR::LEVEL_ERR_LOG));
    }
}

fn clear_space_lines(lines: Vec<String>) -> Vec<String> {
    let mut lines = lines;
    lines.retain(|x| x != "");
    return lines;
}

fn remove_comments(lines: Vec<String>) -> Vec<String> {
    let mut lines = lines;
    for i in 0..lines.len() {
        if lines[i].starts_with("#") || lines[i].starts_with("//") {
            lines[i] = "".to_string();
        }
    }
    return lines;
}

fn read_to_array(path: String) -> Vec<String> {
    let mut filepath = dirs::document_dir().unwrap();
    filepath.push("e2g");
    filepath = filepath.join(path);
    let mut file = match File::open(filepath) {Ok(file) => file, Err(_) => panic!("no such file"),};
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).ok().expect("failed to read!");
    let mut lines: Vec<String> = file_contents.split("\n").map(|s: &str| s.to_string()).collect();
    for i in 0..lines.len() {
        lines[i] = lines[i].trim_start_matches("    ").to_string();
    }
    return lines;
}