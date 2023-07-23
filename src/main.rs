use std::{time::Duration, fs::{File, read_dir, rename}, path::PathBuf};

use chrono::{DateTime, Utc, Timelike};
use crc::{CRC_32_BZIP2, Crc};
use painted_palette_mgr::{Config, Manager, User};

fn main() {
    let mut config = Config {
        command_proxy: "palette-tor".into(),
        directory: "/home/marius/rplace3/ptdeploy".into(),
        archived_dir: "/home/marius/rplace3/archive".into(),
    };
    let now: DateTime<Utc> = Utc::now();

    for file in read_dir(&config.directory).unwrap() {
        let path = file.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") && !file_name.starts_with("package") {
            rename(&path, &PathBuf::from(&config.archived_dir).join(&format!("{}-{}", now.to_rfc3339(), file_name))).unwrap();
        }
    }
    let mut users_file = File::open("./users.json").unwrap();
    let users_from_json: Vec<(String, String)> = serde_json::from_reader(&mut users_file).unwrap();

    let mut count = 0;
    let mut manager = Manager::new(config);
    for user in &users_from_json {
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(user.0.as_bytes());
        let hour = now.time().hour();
        let wake_up_hour = checksum % 24;
        let mut nb = hour as i32 - wake_up_hour as i32;
        if (nb < 0) {
            nb += 24;
        }
        if (nb % 24) < 12 {
            count += 1;
            manager.add_user(User { pass: user.1.clone(), name: user.0.clone() });
        }
    }
    println!("number of users working: {}", count);

    loop {
        manager.start_one_if_possible();
        std::thread::sleep(Duration::from_secs(1));
    }
}
