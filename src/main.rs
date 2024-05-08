/*

    ██████╗ ███████╗███╗   ███╗██╗███╗   ██╗██████╗ ███╗   ██╗███████╗██╗  ██╗ ██████╗
    ██╔══██╗██╔════╝████╗ ████║██║████╗  ██║██╔══██╗████╗  ██║██╔════╝██║ ██╔╝██╔═══██╗     ╱|、
    ██████╔╝█████╗  ██╔████╔██║██║██╔██╗ ██║██║  ██║██╔██╗ ██║█████╗  █████╔╝ ██║   ██║    (˚ˎ 。7
    ██╔══██╗██╔══╝  ██║╚██╔╝██║██║██║╚██╗██║██║  ██║██║╚██╗██║██╔══╝  ██╔═██╗ ██║   ██║     |、˜〵
    ██║  ██║███████╗██║ ╚═╝ ██║██║██║ ╚████║██████╔╝██║ ╚████║███████╗██║  ██╗╚██████╔╝     じしˍ,)ノ
    ╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═════╝ ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝

*/

mod cli;

use chrono::prelude::*;
use chrono::Duration;
use clap::Parser;
use cli::*;
use colored::*;
use colored_json::prelude::*;
use colored_json::to_colored_json;
use humantime::format_duration;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

// yo this regex!?!?
macro_rules! log {
    ($level:expr, $message:expr) => {{
        let symbol_color = match $level {
            "+" => "green",
            "-" => "yellow",
            "!" => "red",
            _ => "white",
        };

        println!(
            "{}{}{} {}",
            "[".bright_black(),
            $level.color(symbol_color),
            "]".bright_black(),
            $message
        )
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        log!("+", $($arg)*)
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        log!("-", $($arg)*)
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        log!("!", $($arg)*)
    }};
}

const REMINDERS_FILE: &str = "REMIND_NEKO.json";

static CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Reminder {
    name: String,
    time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Settings {
    name: String,
    time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    reminders: Vec<Reminder>,
    settings: Vec<Settings>,
}

fn load_reminders() {
    let data = match fs::read_to_string(&REMINDERS_FILE) {
        Ok(data) => data,
        Err(_) => {
            let empty_config = Config {
                reminders: Vec::new(),
                settings: Vec::new(),
            };
            let json = serde_json::to_string_pretty(&empty_config).unwrap();
            fs::write(&REMINDERS_FILE, json).expect("Failed to create reminders file");
            return;
        }
    };
    let config = match serde_json::from_str::<Config>(&data) {
        Ok(config) => config,
        Err(_) => {
            error!("Invalid JSON format in reminders file. Using empty config.");
            Config {
                reminders: Vec::new(),
                settings: Vec::new(),
            }
        }
    };
    CONFIG.set(Mutex::new(config)).unwrap()
}

fn save_reminders() {
    let config = CONFIG.get().unwrap().lock().unwrap().clone();
    std::fs::write(
        &REMINDERS_FILE,
        serde_json::to_string_pretty(&config).unwrap(),
    )
    .unwrap();
    info!("Updated config!")
}

fn new_reminder(name: String, time_str: String) {
    let local: DateTime<Local> = Local::now();
    let duration = parse_duration(&time_str);
    let reminder_time = local + duration;
    let time_left = reminder_time.signed_duration_since(local);

    // handle result otherwise it shits itself
    // mf i wanted to have an 1 liner (ò_óˇ)_╭∩╮
    let std_time_left = match time_left.to_std() {
        Ok(std_duration) => std_duration,
        Err(e) => {
            error!(format!("Error converting duration: {}", e));
            return;
        }
    };

    info!(format!(
        "Setting reminder '{}'. Time left: {} hours ({})",
        name,
        time_left.num_hours(),
        format_duration(std_time_left).to_string()
    ));

    let formatted_time = reminder_time.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut config = CONFIG.get().unwrap().lock().unwrap();
    config.reminders.push(Reminder {
        name: name,
        time: formatted_time,
    });
    drop(config);
    save_reminders();
}

// parse duration from a string like "5hours" or "3days1hour5minutes"
fn parse_duration(input: &str) -> Duration {
    let mut duration = Duration::zero();
    let mut current_number = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else {
            let value = current_number.parse::<i64>().unwrap_or(0);
            current_number.clear();
            match c {
                'd' => duration = duration + Duration::days(value),
                'h' => duration = duration + Duration::hours(value),
                'm' => duration = duration + Duration::minutes(value),
                _ => {}
            }
        }
    }
    duration
}

fn remove_reminder(name: String) {
    let mut config = CONFIG.get().unwrap().lock().unwrap();
    let removed = config.reminders.iter().position(|r| r.name == name);

    if let Some(index) = removed {
        info!(format!("Removing reminder: {}", name));
        config.reminders.remove(index);
        drop(config);
        save_reminders();
    } else {
        error!("Reminder not found!")
    }
}

fn list_reminders() {
    let config = CONFIG.get().unwrap().lock().unwrap().clone();

    info!(format!(
        "Config:\n{}",
        to_colored_json(&config, ColorMode::On).unwrap()
    ));
}

fn start_reminder(name: String) {
    info!(format!("Starting reminder: {}", name));

    let config = CONFIG.get().unwrap().lock().unwrap().clone();
    let matched_conf = config.reminders.iter().position(|r| r.name == name);

    if let Some(index) = matched_conf {
        info!(format!("Matched '{}' at index: {}", name, index));
    } else {
        error!("Reminder not found!");
    }
}

fn main() {
    let args = Cli::parse();
    load_reminders();

    match args.command {
        CliSubcommand::Set { name, time } => {
            new_reminder(name, time);
        }
        CliSubcommand::Remove { name } => {
            remove_reminder(name);
        }
        CliSubcommand::Settings { name, conf } => {
            info!(format!("Setting settings: name={}, config={}", name, conf));
        }
        CliSubcommand::List => {
            list_reminders();
        }
        CliSubcommand::Start { name } => {
            start_reminder(name);
        }
        CliSubcommand::Stop { name } => {
            info!(format!("Stopping reminder: {}", name));
        }
    }
}
/*
  ∧,,,∧
(  ̳• · • ̳)
/    づ♡ read if cute
*/
