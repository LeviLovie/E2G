use dirs;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct BootConf {
    pub pixel_size: usize,
}

pub fn Migrate() {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("e2g");
    let save_folder = path.to_str().unwrap();
    fs::remove_dir_all(save_folder).unwrap();
    println!("Deleted: {:?}", save_folder);
}

pub fn Get_boot_conf() -> BootConf {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("e2g");
    let boot_file = path.join("boot.yaml");
    println!("Booting from: {:?}", boot_file);

    let save_folder = path.to_str().unwrap(); 
    let save_folder_exists: bool = Path::new(save_folder).is_dir();
    if !save_folder_exists {
        println!("Creating save folder: {:?}", save_folder);
        std::fs::create_dir_all(save_folder).unwrap();
        let mut file = match File::create(&boot_file) {
            Ok(file) => file,
            Err(err) => panic!("Failed to create file: {}", err),
        };
        // Writing content to the file
        let content = "pixel_size: 4\n";
        if let Err(err) = file.write_all(content.as_bytes()) {
            panic!("Failed to write to file: {}", err);
        }
    }

    let f = std::fs::File::open(boot_file).expect("Could not open file.");
    let scrape_config: BootConf = serde_yaml::from_reader(f).expect("Could not read values.");
    return BootConf {
        pixel_size: scrape_config.pixel_size,
    };
}