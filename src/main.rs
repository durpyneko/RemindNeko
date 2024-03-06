mod cli;

use chrono::prelude::*;
use clap::Parser;
use cli::*;
use serde::{Deserialize, Serialize};

const REMINDERS_FILE: &str = "reminders.json";
static mut CONFIG: Config = Config {
    reminders: Vec::new(),
    settings: Vec::new(),
};

#[derive(Serialize, Deserialize, Debug)]
struct Reminder {
    time: u32,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    reminders: Vec<Reminder>,
    settings: Vec<Settings>,
}

fn load_reminders() {
    let data = std::fs::read_to_string(&REMINDERS_FILE).unwrap();
    let config = match serde_json::from_str::<Config>(&data) {
        Ok(config) => config,
        Err(_) => Config {
            reminders: Vec::new(),
            settings: Vec::new(),
        },
    };
    unsafe { CONFIG = config };
}

fn save_reminders() {
    unsafe {
        std::fs::write(
            &REMINDERS_FILE,
            serde_json::to_string_pretty(&CONFIG).unwrap(),
        )
        .unwrap();
    }
}

// let mut config = Config {
//     reminders: Vec::new(),
// };
// let reminder = Reminder {
//     time: 0,
//     name: String::from("Example name"),
//     description: String::from("Example description"),
// };
// config.reminders.push(reminder);
// std::fs::write(
//     "reminders.json",
//     serde_json::to_string_pretty(&config).unwrap(),
// )
// .unwrap();

fn new_reminder(name: String, desc: String) {
    println!("Setting reminder: name={}, description={}", name, desc);
    let local: DateTime<Local> = Local::now();
    let time_as_u32 = local.timestamp() as u32;
    unsafe {
        CONFIG.reminders.push(Reminder {
            time: time_as_u32,
            name,
            description: desc,
        })
    };
    save_reminders();
}

fn remove_reminder(name: String) {
    let removed = unsafe { CONFIG.reminders.iter().position(|r| r.name == name) };

    if let Some(index) = removed {
        println!("Removing reminder: name={}", name);
        unsafe {
            CONFIG.reminders.remove(index);
        }
        save_reminders();
    } else {
        println!("Reminder not found.");
    }
}

fn main() {
    let args = Cli::parse();
    load_reminders();

    match args.command {
        CliSubcommand::Set { name, desc } => {
            new_reminder(name, desc);
        }
        CliSubcommand::Remove { name } => {
            remove_reminder(name);
        }
        CliSubcommand::Settings { name, conf } => {
            println!("Setting settings: name={}, config={}", name, conf);
        }
        CliSubcommand::Start { name } => {
            println!("Starting reminder: name={}", name);
        }
        CliSubcommand::Stop { name } => {
            println!("Stopping reminder: name={}", name);
        }
    }
}
