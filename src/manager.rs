use std::{sync::{Arc, Mutex}, thread::{Thread, spawn}, process::Command, os::unix::{thread, process::CommandExt}, time::Duration, path::PathBuf, fs::File};
use nix::{sys::signal::{self, Signal}, unistd::Pid, libc::getpgid};
use wait_timeout::ChildExt;
use crate::{Users, Session, Config, User, PPConfig, PPUser};

pub struct Manager {
    pub users: Arc<Mutex<Users>>,
    config: Config,
    port: u32,
}

impl Manager {
    pub fn new(config: Config) -> Self {
        Self {
            users: Arc::new(Mutex::new(Users::default())),
            config,
            port: 2100
        }
    }

    pub fn add_user(&mut self, user: User) {
        let mut lock = self.users.lock().unwrap();
        lock.free.insert(user);
    }

    pub fn start_one_if_possible(&mut self) {
        let mut lock = self.users.lock().unwrap();
        if lock.free.len() > 0 {
            let mut ppconfig = PPConfig::default();
            let mut to_use = Vec::new();
            for _ in 0..2 {
                if let Some(user) = lock.free.pop_first() {
                    lock.allocated.insert(user.clone());
                    ppconfig.users.insert(user.name.clone(), PPUser::from_user(&user));
                    to_use.push(user);
                }
            }

            //todo: manage port
            let port: u32 = self.port;
            self.port += 1;

            let config_file_path = PathBuf::from(&self.config.directory).join(format!("{}.json", port));
            let mut config_file = File::create(&config_file_path).unwrap();
            serde_json::to_writer(&mut config_file, &ppconfig).unwrap();


            
            let directory_c = self.config.directory.clone();
            let command_proxy_c = self.config.command_proxy.clone();
            let users_c = self.users.clone();

            spawn(move || {
                let mut child = Command::new("bash").arg(command_proxy_c).arg("batch").arg(&port.to_string()).current_dir(&directory_c).spawn().unwrap();
                println!("timeout start");
                child.wait().unwrap();
                
                let mut users_lock = users_c.lock().unwrap();
                for user in to_use {
                    users_lock.allocated.remove(&user);
                    users_lock.free.insert(user);
                }
                return;
            });
        }
    }
}