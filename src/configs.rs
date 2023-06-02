use dirs;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

use crate::ERR;

const BOOT_CONF_CONTENT: &str = "pixel_size: 3\nos: \"os/main.e2g\"\nstart_logo: 1\n";

#[derive(Debug, Serialize, Deserialize)]
pub struct BootConf {
    pub pixel_size: usize,
    pub os: String,
    pub start_logo: i8,
}

pub fn Delete() {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("e2g");
    let save_folder = path.to_str().unwrap();
    let _ = fs::remove_dir_all(save_folder);
    ERR::err_catch(ERR::Err::new(format!("Deleted: {:?}", save_folder).as_str(), ERR::LEVEL_INFO));
}

pub fn Get_boot_conf() -> BootConf {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("e2g");
    let boot_file = path.join("boot.yaml");
    let f = std::fs::File::open(boot_file).expect("Could not open file.");
    let scrape_config: BootConf = serde_yaml::from_reader(f).expect("Could not read values.");
    return BootConf {
        pixel_size: scrape_config.pixel_size,
        os: scrape_config.os,
        start_logo: scrape_config.start_logo,
    };
}

pub fn Check(patch: bool) {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("e2g");

    check_directory_exists(path.to_str().unwrap(), patch);
    check_directory_exists(path.join("os").to_str().unwrap(), patch);

    check_file_exists(path.join("boot.yaml").to_str().unwrap(), BOOT_CONF_CONTENT, patch, false);
}

fn check_directory_exists(path: &str, patch: bool) {
    let save_folder = path; 
    let save_folder_exists: bool = Path::new(save_folder).is_dir();
    if !save_folder_exists {
        if patch {
            std::fs::create_dir_all(save_folder).unwrap();
            ERR::err_catch(ERR::Err::new(format!("Directory {:?} is not exists, patched", save_folder).as_str(), ERR::LEVEL_INFO));
        } else {
            ERR::err_catch(ERR::Err::new(format!("Directory {:?} is not exists, continue checking is unreal (use \"patch\" flag to patch it)", save_folder).as_str(), ERR::LEVEL_ERR_FATAL));
        }
    }
}

fn check_file_exists(path: &str, content: &str, patch: bool, static_file: bool) {
    // Check, that it is exists
    let file_exists: bool = Path::new(path).exists();
    if file_exists {
        // Check content
        ERR::err_catch(ERR::Err::new(format!("File {:?} - exists", path).as_str(), ERR::LEVEL_INFO));
        if !static_file {
            let contents = fs::read_to_string(path)
                .expect("Should have been able to read the file");
            if contents == content {
                ERR::err_catch(ERR::Err::new(format!("File {:?} - content is correct", path).as_str(), ERR::LEVEL_INFO));
            } else {
                if patch {
                    // Remove file
                    let err = fs::remove_file(path);
                    // Recreate file
                    let mut file = match File::create(path) {
                        Ok(file) => file,
                        Err(err) => panic!("Failed to create file: {:?}", err),
                    };
                    // Write to file
                    let mut fileRef = OpenOptions::new().append(true).open(path).expect("Unable to open file");   
                    fileRef.write_all(content.as_bytes()).expect("write failed");
                    ERR::err_catch(ERR::Err::new(format!("File {:?} - content is not correct, patched", path).as_str(), ERR::LEVEL_WARN));
                } else {
                    ERR::err_catch(ERR::Err::new(format!("File {:?} - content is not correct (use \"patch\" flag to patch it)", path).as_str(), ERR::LEVEL_WARN));
                }
            }
        }
    } else {
        if patch {
            // Creating
            let mut file = match File::create(path) {
                Ok(file) => file,
                Err(err) => panic!("Failed to create file: {:?}", err),
            };
            // Write to file
            let mut fileRef = OpenOptions::new().append(true).open(path).expect("Unable to open file");   
            fileRef.write_all(content.as_bytes()).expect("write failed");
            ERR::err_catch(ERR::Err::new(format!("File {:?} - not exists, patched", path).as_str(), ERR::LEVEL_INFO));
        } else {
            ERR::err_catch(ERR::Err::new(format!("File {:?} - not exists (use \"patch\" flag to patch it)", path).as_str(), ERR::LEVEL_WARN));
        }
    }
}
