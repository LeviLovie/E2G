use colored::Colorize;
use chrono::prelude::*;

// MSG: String err message
// ERR: Result<(), Error>
// LEVEL: i8 err level (can use var)
pub struct Err {
    pub msg: String,
    pub level: i8,
}

impl Err {
    pub fn new(msg: &str, level: i8) -> Err {
        Err {
            msg: msg.to_string(),
            level: level,
        }
    }
}

pub const LEVEL_INFO: i8 = 0;
pub const LEVEL_WARN: i8 = 1;
pub const LEVEL_ERR_LOG: i8 = 2;
pub const LEVEL_ERR_RESTART_PROCESS: i8 = 3;
pub const LEVEL_ERR_REBOOT: i8 = 4;
pub const LEVEL_ERR_FATAL: i8 = 5;

pub fn err_catch(err: Err) {
    print_err(&err);
    if err.level == LEVEL_ERR_FATAL {std::process::exit(0);}
}

pub fn print_err(err: &Err) {
    let err_type: String;

    if      err.level == LEVEL_INFO                {err_type = "INFO".green().to_string();}
    else if err.level == LEVEL_WARN                {err_type = "WARN".yellow().to_string();}
    else if err.level == LEVEL_ERR_LOG             {err_type = "ERR_LOG".red().to_string();}
    else if err.level == LEVEL_ERR_RESTART_PROCESS {err_type = "ERR_RESTART_PROCESS".red().to_string();}
    else if err.level == LEVEL_ERR_REBOOT          {err_type = "ERR_REBOOT".on_red().to_string();}
    else if err.level == LEVEL_ERR_FATAL           {err_type = "ERR_FATAL".on_red().to_string();}
    else                                           {err_type = "ERR_UNKNOWN".purple().to_string();}

    println!("{:<30}{:<30}{}", Local::now().format("%Y-%m-%dT%H:%M:%S").to_string().blue().to_string(), err_type, err.msg);
}
